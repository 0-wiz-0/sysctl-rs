extern crate sysctl;

// Import the trait
use sysctl::Sysctl;

#[cfg(any(target_os = "macos", target_os = "freebsd"))]
const CTLNAMES: &[&str] = &["kern.ostype"];

#[cfg(any(target_os = "linux", target_os = "android"))]
const CTLNAMES: &[&str] = &["kernel.ostype", "kernel/ostype", "/proc/sys/kernel/ostype"];

fn print_ctl(ctlname: &str) -> Result<(), sysctl::SysctlError> {
    println!("Reading '{}'", ctlname);
    let ctl = try!(sysctl::Ctl::new(ctlname));
    let description = try!(ctl.description());
    println!("Description: {}", description);
    let val_enum = try!(ctl.value());
    println!("Value: {}", val_enum);
    Ok(())
}

fn main() {
    for ctlname in CTLNAMES {
        print_ctl(ctlname).unwrap_or_else(|e: sysctl::SysctlError| {
            eprintln!("Error: {:?}", e);
        });
    }
}
