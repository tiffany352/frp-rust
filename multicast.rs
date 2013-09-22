use std::task;
use std::rt::comm::*;

pub struct MultiPort<T>(Chan<Chan<T>>);

impl<T:Clone+Send> MultiPort<T> {
    pub fn new(port: Port<T>) -> MultiPort<T> {
        let (chanport, chanchan) = stream();
        let (xport, xchan): (Port<Either<T,Chan<T>>>, Chan<Either<T,Chan<T>>>) = stream();
        let xchan = SharedChan::new(xchan);
        do task::spawn_with((xchan.clone(), port)) |(chan, port)| {
            loop {
                match port.try_recv() {
                    Some(x) => chan.send(Left(x)),
                    None => break
                }
            }
        }
        do task::spawn_with((xchan, chanport)) |(chan, port)| {
            loop {
                match port.try_recv() {
                    Some(x) => chan.send(Right(x)),
                    None => break
                }
            }
        }
        do task::spawn_with(xport) |port| {
            let mut chans: ~[Chan<T>] = ~[];
            loop {
                match port.try_recv() {
                    Some(Left(x)) => for c in chans.iter() {
                        c.send(x.clone())
                    },
                    Some(Right(x)) => chans.push(x),
                    None => break
                }
            }
        }
        MultiPort(chanchan)
    }
    pub fn add(&self, chan: Chan<T>) {
        self.send(chan)
    }
}

