use nannou::prelude::*;

pub fn texture_from_image_buffer(
    app: &App,
    frame: &Frame,
    image_buffer: &imageproc::image::ImageBuffer<imageproc::image::Rgba<u8>, Vec<u8>>,
) -> wgpu::Texture {
    let boundary = app.window_rect();

    let texture = wgpu::TextureBuilder::new()
        .size([boundary.w() as u32, boundary.h() as u32])
        .format(wgpu::TextureFormat::Rgba8Unorm)
        .usage(wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING)
        .build(app.main_window().device());

    let flat_samples = image_buffer.as_flat_samples();

    texture.upload_data(
        app.main_window().device(),
        &mut *frame.command_encoder(),
        &flat_samples.as_slice(),
    );

    texture
}
