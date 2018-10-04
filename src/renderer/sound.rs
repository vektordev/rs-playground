use rodio;
use rodio::Sink;
use rodio::buffer::SamplesBuffer;
use std::f32::consts::PI;

pub fn test() {
    let device = rodio::default_output_device().unwrap();
    let mut sink = Sink::new(&device);

    // Add a dummy source of the sake of the example.
    let buff_size = 440000;
    let mut timestep = 0;
    loop {
        let mut buff = Vec::new();
        for x in 0..buff_size{
            let time = ((timestep + x) as f32) / 44100f32;
            buff.push(sound_src(time));
        }
        timestep += buff_size;
        let source = SamplesBuffer::new(1, 44100, buff);
        //let source = rodio::source::SineWave::new(440);
        //sink.sleep_until_end();
        sink.set_volume(0.3f32);
        sink.append(source);
    }
}

pub fn sound_src(time: f32 ) -> f32 {
    //let a = sine(440f32, time);
    let a = if time < 2f32 {0f32} else {sine(220f32, time)};
    let b = if time < 4f32 {0f32} else {sine(440f32, time)};
    let c = if time < 6f32 {0f32} else {sine(660f32, time)};
    let d = if time < 8f32 {0f32} else {sine(880f32, time)};
    //let c = sine(660f32, time);
    //let d = sine(880f32, time);
    return a+b+c+d
    //return (time*440f32 * 2f32 * PI).sin()
}

pub fn sine(freq: f32, time: f32) -> f32{
    return (time*freq*2f32*PI).sin()
}