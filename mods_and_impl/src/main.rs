mod sample_module;
use sample_module::SampleImpl;

fn main() {
  let sample = SampleImpl::new("Anoop".to_string());
  sample.hello();
}
