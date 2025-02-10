
use std::ops::Deref;
use std::sync::atomic::Ordering;
use std::sync::atomic::Ordering::{Relaxed,SeqCst};
use crossbeam_epoch::{self as epoch, Atomic, Guard, Owned, Shared};
use rand::prelude::*;
use rand_distr::StandardGeometric;
use rand_unique::{RandomSequenceBuilder,RandomSequence};


// struct SkipNode{
//     data: i32,
//     pointers: Vec<Atomic<SkipNode>>
// }
//
//
// struct SkipList{
//     head: SkipNode,
// }
//
// impl SkipList {
//     fn new() -> Self {
//         SkipList{head:
//             SkipNode{
//                 data:0,
//                 pointers: vec![]
//             }
//         }
//     }
//     fn insert(&mut self, tower: SkipNode) {
//
//
//     }
// }

#[derive(Debug)]
struct SkipNode {
    data: i32,
    p:Vec<Atomic<SkipNode>>
}
#[derive(Debug)]
struct SkipList {
    head: Atomic<SkipNode>,
    count: i32
}

impl SkipList {
    fn new() -> Self{
        SkipList {
            head: Atomic::new(SkipNode {
                data: i32::MIN,
                p:vec![Atomic::null();2],
            }),
            count: 1
        }
    }
    unsafe fn insert(&mut self, guard: &Guard, ele:i32){
        let v = StandardGeometric.sample(&mut rand::rng()) as usize;
        let skp = Atomic::new(SkipNode {
            data: ele,
            p:vec![Atomic::null();v + 1],
        });
        self.count += 1;

        let mut refer = &self.head;

        let mut level = refer.load(Relaxed, guard).deref().p.len();
        if v +1 > level{
            for _ in 0..(v +1 - level){
                refer.load(Relaxed, guard).deref_mut().p.push(Atomic::null());
            }
        }
        while level > 0{
            let cur = &refer
                .load(Relaxed, guard).deref().p[level-1];
            if cur.load(Relaxed, guard).is_null(){
                if skp.load(Relaxed, guard).deref().p.len() >= level{
                    skp.load(Relaxed, guard).deref_mut().p[level -1].swap(Shared::null(), Relaxed, guard);
                    cur.store(skp.load(Relaxed, guard), Relaxed);
                }
                level = level-1;
                continue;
            }
            let c = cur.load(Relaxed, guard).deref().data;
            if c > skp.load(Relaxed, guard).deref().data {
                if skp.load(Relaxed, guard).deref().p.len() >= level{
                    skp.load(Relaxed, guard).deref_mut().p[level -1].swap(cur.load(Relaxed, guard).into_owned(), Relaxed, guard);

                    //let s =  skp.load(Relaxed, guard).deref().p[level-1].load(Relaxed, guard).deref();

                    cur.swap(skp.load(Relaxed, guard).into_owned(), Relaxed,guard);

                }
                level = level-1;
            }else{
                refer = cur;

            }
        }
        // println!("END");
    }
    unsafe fn print(self, guard: &Guard){
        //let guard = &epoch::pin();
        let mut refer = &self.head;
        let check = &refer.load(SeqCst,guard).deref().p[0];
        let mut count = self.count;
        while count > 0{
            println!("data: {:?}", refer.load(SeqCst,guard).deref().data );
            //println!("tower: {:?}",&refer.load(SeqCst,guard).deref());
            refer = &refer.load(SeqCst,guard).deref().p[0];
            count -=1;

        }

    }
}

unsafe fn sk_test(n: i32){
    let guard = epoch::pin();
    let mut skp = SkipList::new();
    (0..n).for_each(|x| unsafe {
        if n == 5000{
            println!("9!")
        }
        skp.insert(&guard,x)
    });
    skp.print(&guard);
}

fn main() {
    // println!("Hello, world!");
    //
    //
    //
    // let guard = &epoch::pin();
    // use crossbeam_epoch::{self as epoch, Atomic};
    // let mut s = SkipList::new();
    // let i = Atomic::new(SkipNode {data:0, p:vec![Atomic::null(); 2]});
    // let i2 = Atomic::new(SkipNode {data:-1, p:vec![Atomic::null(); 1]});
    // let i3 = Atomic::new(SkipNode {data:-100, p:vec![Atomic::null(); 1]});
    // println!("i2 {:?}",i2);
    // unsafe { s.insert(guard,0); }
    // unsafe { s.insert(guard,1); }
    // unsafe { s.insert(guard,-1); }
    // println!("yayyyy");
    // //println!("s{:?}", s);
    // unsafe { s.print(guard) }
    unsafe { sk_test(100000); }
    use std::sync::atomic::Ordering::SeqCst;


    // let a = Atomic::new(Stupid{ data:0 });
    // let guard = &epoch::pin();
    // let mut p = a.load(SeqCst, guard);
    // unsafe { p.deref_mut().data = 1; }
    // unsafe { println!("{:?}", p.deref().data); }
}
