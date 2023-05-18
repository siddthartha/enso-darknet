use anyhow::Result;
use tch::Tensor;

fn main() -> Result<()>
{
    let t1 = Tensor::from_slice(&[3, 1, 4, 1, 5]);
    let t2 = t1 * 3.1415926535897932385;

    t2.print();

    return Ok(());
}
