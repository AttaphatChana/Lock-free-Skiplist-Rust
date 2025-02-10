use std::sync::atomic::Ordering;
use std::sync::atomic::Ordering::{Acquire, Relaxed, SeqCst};
use crossbeam_epoch::{self as epoch, Atomic, Guard, Owned, Shared};
use rand::distr::Distribution;
use rand::Rng;
use rand_distr::StandardGeometric;

#[derive(Debug)]
struct SkipNode {
    data: i32,
    p:Vec<Atomic<SkipNode>>
}
#[derive(Debug)]
pub struct SkipList {
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

        let level = refer.load(Relaxed, guard).deref().p.len();
        if v +1 > level{
            for _ in 0..(v +1 - level){
                refer.load(Relaxed, guard).deref_mut().p.push(Atomic::null());
            }
        }
        let mut level = refer.load(Relaxed, guard).deref().p.len();
        while level > 0{
            let cur = &refer
                .load(Acquire, guard).deref().p[level-1];
            if cur.load(Relaxed, guard).is_null(){

                if skp.load(Relaxed, guard).deref().p.len() >= level{
                    skp.load(Relaxed, guard).deref_mut().p[level -1].swap(Shared::null(), Relaxed, guard);
                    cur.store(skp.load(Relaxed, guard), Relaxed);
                }
                level = level-1;
                continue;
            }
            let c = cur.load(Ordering::Relaxed, guard).deref().data;
            //let z  = cur.load(SeqCst, guard).into_owned();
            //level = 0;
            if c > skp.load(Relaxed, guard).deref().data {
                if skp.load(Relaxed, guard).deref().p.len() >= level{
                    skp.load(Acquire, guard).deref_mut().p[level -1].swap(cur.load(Relaxed, guard).into_owned(), Relaxed, guard);

                    let s =  skp.load(Relaxed, guard).deref().p[level-1].load(Relaxed, guard).deref();

                    cur.swap(skp.load(Acquire, guard).into_owned(), Relaxed,guard);

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
            // println!("data: {:?}", refer.load(SeqCst,guard).deref().data );
            // println!("tower: {:?}",&refer.load(SeqCst,guard).deref());
            refer = &refer.load(SeqCst,guard).deref().p[0];
            count -=1;

        }

    }
}

pub fn sk_test(seq: Vec<i32>){
    let guard = epoch::pin();
    let mut skp = SkipList::new();
    // let mut rng = rand::thread_rng();
    // let sequence: Vec<i32> = (0..n).map(|_| rng.random_range(-n..n)).collect();
    seq.iter().for_each(|x| unsafe {
        skp.insert(&guard,*x)
    })
}