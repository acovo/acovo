
/// $r: return value, $s: success value, $e:action , $t: timeout in seconds
#[cfg(feature = "syncall")]
#[macro_export]
macro_rules! syncall_with_timeout {
    ($r:expr,$s:expr,$e:expr,$t:expr) => {
        let tm_start = Instant::now();
        while(tm_start.elapsed().as_secs() <= $t as u64) {
            let ret=$e;
            if ret==$s {
                $r=ret;
                break;
            }else {
                continue;
            }
        }
    };
}

#[cfg(feature = "syncall")]
#[macro_export]
macro_rules! syncall_with_signal_timeout {
    ($r:expr,$s:expr,$e:expr,$t:expr) => {
        let tm_start = Instant::now();
        while(tm_start.elapsed().as_secs() <= $t as u64) {
            let ret=$e;
            if unsafe {*$s.lock().unwrap()}==true {                
                $r=ret;
                //clear lock flag.
                unsafe {
                    let mut value = $s.lock().unwrap();
                    *value=false;
                }
                break;
            }else {
                continue;
            }
        }
    };
}

#[cfg(test)]
#[cfg(feature = "syncall")]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    use std::time::Instant;
    use std::sync::Mutex;

    static mut SignalExit:Mutex::<bool>=Mutex::new(false);

    fn sleep_test(timeout:u8,info:&mut u8)->u8 {
        println!("sleep_test {}",timeout);
        thread::sleep(Duration::from_secs(1));
        *info=3 as u8;
        1
    }

    #[test]
    fn test_syncall() {

        thread::spawn(|| loop {
            thread::sleep(Duration::from_secs(8));
            unsafe {
                let mut value = SignalExit.lock().unwrap();
                *value=true;
            }
        });

        let mut ret = 0;
        let mut ifo=2;
        let s = 1;
        syncall_with_timeout!(ret,1,sleep_test(s,&mut ifo),10);
        println!("{}",ret);

        syncall_with_signal_timeout!(ret,SignalExit,sleep_test(s,&mut ifo),10);
        println!("{}",ret);
    }
}
