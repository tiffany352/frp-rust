use std::task;
//use std::rt::comm::*;
//use multicast::*;
use std::comm::channel;
use multicast::MultiReceiver;

pub struct Signal<T> {
    receiver: MultiReceiver<T>,
}

enum Either<L,R> {
    Left(L),
    Right(R),
}

pub fn lift<T: Send + Clone, U: Send + Clone>(sig: &Signal<T>, func: extern fn(T) -> U) -> Signal<U> {
    let (chan, port) = channel();
    sig.receiver.add(chan);
    let (mchan, mport) = channel();
    task::spawn(proc() {
        loop {
            match port.recv_opt() {
                Ok(x) => { mchan.send(func(x)); }
                Err(()) => { break; }
            }
        }
    });
    Signal {
        receiver: MultiReceiver::new(mport),
    }
}

pub fn lift2<T1:Send+Clone,T2:Send+Clone,U:Send+Clone>(sig1: &mut Signal<T1>, sig2: &Signal<T2>, func: extern fn(T1,T2) -> U) -> Signal<U> {
    let (chan, port) = channel();
    let (xchan, xport): (Sender<Either<T1,T2>>, Receiver<Either<T1,T2>>) = channel();
    let (mchan1, mport1) = channel();
    let (mchan2, mport2) = channel();
    sig1.receiver.add(mchan1);
    sig2.receiver.add(mchan2);

    let xchan1 = xchan.clone();
    task::spawn(proc() {
        loop {
            match mport1.recv_opt() {
                Ok(x) => {
                    println!("port1");
                    xchan1.send(Left(x));
                }
                Err(()) => { break; }
            }
        }
    });

    let xchan2 = xchan.clone();
    task::spawn(proc() {
        loop {
            match mport2.recv_opt() {
                Ok(x) => {
                    println!("port2");
                    xchan2.send(Right(x));
                },
                Err(()) => { break; }
            }
        }
    });

    task::spawn(proc() {
        let mut x = None;
        let mut y = None;
        loop {
            match xport.recv_opt() {
                Ok(Left(v)) => x = Some(v),
                Ok(Right(v)) => y = Some(v),
                Err(()) => {
                    println!("???");
                    break;
                }
            }
            match (x.clone(), y.clone()) {
                (Some(a), Some(b)) => {
                    chan.send(func(a, b));
                    x = None;
                    y = None;
                }
                _ => ()
            }
        }
    });

    Signal {
        receiver: MultiReceiver::new(port)
    }
}

fn constant<T:Clone+Send>(val: T) -> (Sender<()>, Signal<T>) {
    let (chan, port) = channel();
    let (mchan, mport) = channel();
    task::spawn(proc() {
        port.recv();
        mchan.send(val.clone());
    });
    (chan, Signal { receiver: MultiReceiver::new(mport) })
}
