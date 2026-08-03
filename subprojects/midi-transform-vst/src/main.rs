use midi_transform_vst::MidiTransform;

fn main() {
    nih_plug::backend::vst3::main::<MidiTransform>();
}