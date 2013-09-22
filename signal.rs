use std::task;
use std::rt::comm::*;
use multicast::*;

pub struct Signal<T>(MultiPort<T>);

pub fn lift<T:Send+Clone,U:Send+Clone>(sig: &Signal<T>, func: ~fn(T) -> U) -> Signal<U> {
    let (port, chan) = stream();
    sig.add(chan);
    let (mport, mchan) = stream();
    do task::spawn_with((port, mchan, func)) |(port, chan, func)| {
        loop {
            match port.try_recv() {
                Some(x) => chan.send(func(x)),
                None => break
            }
        }
    }
    Signal(MultiPort::new(mport))
}

pub fn lift2<T1:Send+Clone,T2:Send+Clone,U:Send+Clone>(sig1: &Signal<T1>, sig2: &Signal<T2>, func: ~fn(T1,T2) -> U) -> Signal<U> {
    let (port, chan) = stream();
    let (xport, xchan): (Port<Either<T1,T2>>, Chan<Either<T1,T2>>) = stream();
    let (mport1, mchan1) = stream();
    let (mport2, mchan2) = stream();
    sig1.add(mchan1);
    sig2.add(mchan2);
    let xchan = SharedChan::new(xchan);
    do task::spawn_with((mport1, xchan.clone())) |(port, chan)| {
        loop {
            match port.try_recv() {
                Some(x) => {println("port1");chan.send(Left(x))},
                None => break
            }
        }
    }
    do task::spawn_with((mport2, xchan)) |(port, chan)| {
        loop {
            match port.try_recv() {
                Some(x) => {println("port2");chan.send(Right(x))},
                None => break
            }
        }
    }
    do task::spawn_with((xport, chan, func)) |(port, chan, func)| {
        let mut x = None;
        let mut y = None;
        loop {
            match port.try_recv() {
                Some(Left(v)) => x = Some(v),
                Some(Right(v)) => y = Some(v),
                None => {println("???"); break}
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
    }
    Signal(MultiPort::new(port))
}

fn constant<T:Clone+Send>(val: T) -> (Chan<()>, Signal<T>) {
    let (port, chan) = stream();
    let (mport, mchan) = stream();
    do task::spawn_with((port, mchan)) |(port, chan)| {
        port.recv();
        chan.send(val.clone());
    }
    (chan, Signal(MultiPort::new(mport)))
}

