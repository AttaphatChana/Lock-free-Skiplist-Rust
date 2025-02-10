use std::sync::atomic::Ordering::{Acquire, Relaxed, SeqCst};
use crossbeam_epoch::{self as epoch, Atomic, Guard, Owned, Shared};
use rand::Rng;
use crate::something::SkipList;
use skiplist::OrderedSkipList;

pub mod something{
    use std::sync::atomic::Ordering;
    use std::sync::atomic::Ordering::{Acquire, Relaxed, SeqCst};
    use crossbeam_epoch::{Atomic, Guard, Shared};
    use rand::distr::Distribution;
    use rand_distr::StandardGeometric;

    #[derive(Debug)]
    pub struct SkipNode {
        data: i32,
        p:Vec<Atomic<SkipNode>>
    }


    #[derive(Debug)]
    pub struct SkipList {
        pub head: Atomic<SkipNode>,
        pub count: i32
    }




    impl SkipList {
        pub fn new() -> Self{
            SkipList {
                head: Atomic::new(SkipNode {
                    data: i32::MIN,
                    p:vec![Atomic::null();2],
                }),
                count: 1
            }
        }
        pub unsafe fn insert(&mut self, guard: &Guard, ele:i32){
            let v = StandardGeometric.sample(&mut rand::rng()) as usize;
            let skp = Atomic::new(SkipNode {
                data: ele,
                p:vec![Atomic::null();v + 1],
            });
            self.count += 1;

            let mut refer = &self.head;

            let mut level = refer.load(Relaxed, guard).deref().p.len();
            let max_level = v +1;
            if v +1 > level{
                for _ in level..=max_level{
                    refer.load(Relaxed, guard).deref_mut().p.push(Atomic::null());
                }
                for i in level..(v +1){

                    refer.load(Relaxed, guard).deref().p[i].store(skp.load(Relaxed, guard), Relaxed);
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
                let c = cur.load(Ordering::Relaxed, guard).deref().data;
               
                if c == skp.load(Relaxed, guard).deref().data{
                    break;
                }
                if c > skp.load(Relaxed, guard).deref().data {
                    if skp.load(Relaxed, guard).deref().p.len() >= level{
                        skp.load(Relaxed, guard).deref_mut().p[level -1].swap(cur.load(Relaxed, guard).into_owned(), Relaxed, guard);

                        cur.swap(skp.load(Relaxed, guard).into_owned(), Relaxed,guard);

                    }
                    level = level-1;
                }else{
                    refer = cur;

                }
            }
            // println!("END");
        }
        pub unsafe fn search_n(&self, guard: &Guard,data: i32) -> bool {
            let mut refer = &self.head;
            let mut level = refer.load(Relaxed, guard).deref().p.len();

            while level > 0{
                let cur = &refer
                    .load(Relaxed, guard).deref().p[level-1];
                if cur.load(Relaxed, guard).is_null(){
                    level = level-1;
                    continue;
                }
                let c = cur.load(Relaxed, guard).deref().data;
                if c > data {
                    level = level-1;
                }else if c < data {
                    refer = cur;
                }else{
                    return true
                }
            }
            false

        }
        unsafe fn print(self, guard: &Guard){
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

}

pub fn import_sk(n:i32){
    let mut skp = OrderedSkipList::new();
    let mut rng = rand::rng();
    let sequence: Vec<i32> = (0..n).map(|_| rng.random_range(-n..n)).collect();
    sequence.iter().for_each(|x| unsafe {
        skp.insert(*x)
    })
}

pub fn sk_test(n:i32){
    let guard = epoch::pin();
    let mut skp = SkipList::new();
    let mut rng = rand::thread_rng();
    let sequence: Vec<i32> = (0..n).map(|_| rng.random_range(-n..n)).collect();
    sequence.iter().for_each(|x| unsafe {
        skp.insert(&guard,*x)
    })
}