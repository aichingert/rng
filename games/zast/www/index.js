async function init() {
    let canvas = document.getElementById("app");

    const displayWidth  = canvas.clientWidth;
    const displayHeight = canvas.clientHeight;

    // Check if the canvas is not the same size.
    const needResize = canvas.width  !== displayWidth || canvas.height !== displayHeight;

    if (needResize) {
        canvas.width  = displayWidth;
        canvas.height = displayHeight;
    }

    const gl = canvas.getContext("webgl2");

    if (gl === null) {
        throw new Error("Failed to initialize webgl context");
    }

    let vertices = [
        0.0, 0.15, 0.0,
        -0.15, -0.15, 0.0,
        0.15, -0.15, 0.0,
    ];

    let indices = [0,1,2];

    let vertex_buffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, vertex_buffer);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW);

    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, null);

    let idx_buffer = gl.createBuffer();
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, idx_buffer);
    gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, new Uint16Array(indices), gl.STATIC_DRAW);

    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, null);
    
    let vert_src = `
    attribute vec3 coordinates;
    void main(void) {
        gl_Position = vec4(coordinates, 1.0);
    }
    `;
    let frag_src = `
    void main(void) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 0.1);
    }
    `

    let vert_shader = gl.createShader(gl.VERTEX_SHADER);
    let frag_shader = gl.createShader(gl.FRAGMENT_SHADER);

    gl.shaderSource(vert_shader, vert_src);
    gl.shaderSource(frag_shader, frag_src);

    gl.compileShader(vert_shader);
    gl.compileShader(frag_shader);

    let program = gl.createProgram();

    gl.attachShader(program, vert_shader);
    gl.attachShader(program, frag_shader);
    gl.linkProgram(program);
    gl.useProgram(program);

    gl.bindBuffer(gl.ARRAY_BUFFER, vertex_buffer);
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, idx_buffer);

    let coord = gl.getAttribLocation(program, "coordinates");

    gl.vertexAttribPointer(coord, 3, gl.FLOAT, false, 0, 0);

    gl.enableVertexAttribArray(coord);

    gl.clearColor(0.0, 0.0, 0.0, 1.0);
    gl.enable(gl.DEPTH_TEST);

    gl.clear(gl.COLOR_BUFFER_BIT);

    gl.viewport(0, 0, canvas.width, canvas.height);
    gl.drawElements(gl.TRIANGLES, indices.length, gl.UNSIGNED_SHORT, 0);
}

window.onload = init;
