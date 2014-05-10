use std::task;
use std::comm::{Sender, Receiver, channel};

pub struct MultiReceiver<T> {
    sender: Sender<Sender<T>>,
}

enum Either<L,R> {
    Left(L),
    Right(R),
}

impl<T: Clone + Send> MultiReceiver<T> {
    pub fn new(port: Receiver<T>) -> MultiReceiver<T> {
        let (chanchan, chanport) = channel();
        let (xchan, xport) = channel();

        let xchan1 = xchan.clone();
        task::spawn(proc() {
            loop {
                match port.recv_opt() {
                    Ok(x) => { xchan1.send(Left(x)); }
                    Err(()) => { break; }
                }
            }
        });

        let xchan2 = xchan.clone();
        task::spawn(proc() {
            loop {
                match chanport.recv_opt() {
                    Ok(x) => { xchan2.send(Right(x)); }
                    Err(()) => { break; }
                }
            }
        });

        task::spawn(proc() {
            let mut chans: Vec<Sender<T>> = Vec::new();
            loop {
                match xport.recv_opt() {
                    Ok(Left(x)) => {
                        for c in chans.iter() {
                            c.send(x.clone())
                        }
                    }
                    Ok(Right(x)) => { chans.push(x); }
                    Err(()) => { break; }
                }
            }
        });

        MultiReceiver { sender: chanchan }
    }

    pub fn add(&self, chan: Sender<T>) {
        self.sender.send(chan)
    }
}

