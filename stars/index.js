const SHADERS = `
struct WeirdTransform {
    position: vec2f,
}

struct VertexOut {
    @builtin(position) position : vec4f,
    @location(0) color : vec4f
}

@binding(0) @group(0) var<uniform> transform: WeirdTransform;

@vertex
fn vertex_main(@location(0) position: vec4f,
               @location(1) color: vec4f) -> VertexOut
{
    var output : VertexOut;
    output.position = position + vec4(transform.position, 0, 0);
    output.color = color;
    return output;
}

@fragment
fn fragment_main(fragData: VertexOut) -> @location(0) vec4f
{
    return fragData.color;
}
`;


class Scene {
    constructor(device, pipeline, bind_group, ub, ctx) {
        this.device = device;
        this.pipeline = pipeline;
        this.bind_group = bind_group;
        this.vb = null;
        this.ub = ub;
        this.ctx = ctx;
        this.vertice_count = 0;
    }

    setup_player_model() {
        const vertices = new Float32Array([
            0.0, 0.6, 0, 1, // pos
            1, 0, 0, 1,     // col
            -0.5,-0.6, 0, 1, 
            0, 1, 0, 1, 
            0.5,-0.6, 0, 1, 
            0, 0, 1, 1,
        ]);

        this.vb = this.device.createBuffer({
            size: vertices.byteLength,
            usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
        });

        this.device.queue.writeBuffer(this.vb, 0, vertices, 0, vertices.length); 
        this.vertice_count = vertices.length / 8;
    }

    /* TODO: update player position
        device.queue.writeBuffer(ub, 0, new Float32Array([0.1, -0.1]));
    */

    draw_player(at) {
        this.device.queue.writeBuffer(this.ub, 0, new Float32Array(at));
        const command_encoder = this.device.createCommandEncoder();
        const clear_color = { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };

        const render_pass_descriptor = {
            colorAttachments: [{
                clearValue: clear_color,
                loadOp: "clear",
                storeOp: "store",
                view: this.ctx.getCurrentTexture().createView(),
            }],
        };

        const pass_encoder = command_encoder.beginRenderPass(render_pass_descriptor);
        pass_encoder.setPipeline(this.pipeline);
        pass_encoder.setVertexBuffer(0, this.vb);
        pass_encoder.setBindGroup(0, this.bind_group);
        pass_encoder.draw(this.vertice_count);
        pass_encoder.end();
        this.device.queue.submit([command_encoder.finish()]);
    }
}

async function setup_wgpu(context) {
    // TODO: handle error cases
    const adapter = await navigator.gpu.requestAdapter();
    const device = await adapter.requestDevice();
    const shaders = device.createShaderModule({code: SHADERS});

    context.configure({
        device: device,
        format: navigator.gpu.getPreferredCanvasFormat(),
        alphaMode: "premultiplied",
    });

    const vertex_buffer = {
        attributes: [
            { shaderLocation: 0, offset:  0, format: "float32x4" },
            { shaderLocation: 1, offset: 16, format: "float32x4" },
        ],
        arrayStride: 32,
        stepMode: "vertex",
    };
    const uniform_buffer = device.createBuffer({
        size: 8, 
        usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    });

    const bind_group_layout = device.createBindGroupLayout({
        entries: [{
            binding: 0,
            visibility: GPUShaderStage.VERTEX,
            buffer: {},
        }],
    });
    const bind_group = device.createBindGroup({
        layout: bind_group_layout,
        entries: [{
            binding: 0,
            resource: { buffer: uniform_buffer },
        }],
    });

    const pipeline_layout = device.createPipelineLayout({
        bindGroupLayouts: [bind_group_layout],
    });

    const pipeline_descriptor = {
        vertex: {
            module: shaders,
            entryPoint: "vertex_main",
            buffers: [vertex_buffer],
        },
        fragment: {
            module: shaders,
            entryPoint: "fragment_main",
            targets: [{
                format: navigator.gpu.getPreferredCanvasFormat(),
            }],
        },
        primitive: { topology: "triangle-list" },
        layout: pipeline_layout,
    };

    const pipeline = device.createRenderPipeline(pipeline_descriptor);

    return new Scene(device, pipeline, bind_group, uniform_buffer, context);
}

async function init() {
    let canvas = document.getElementById("app");
    canvas.width = canvas.clientWidth;
    canvas.height = canvas.clientHeight;
    const ctx = canvas.getContext("webgpu");

    let scene = await setup_wgpu(ctx);
    scene.setup_player_model();

    while (true) {
        await new Promise(r => setTimeout(r, 1));
        scene.draw_player([0, 0]);
    }
}

window.onload = init;
