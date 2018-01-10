use std::collections::VecDeque;

pub trait SampleType: Copy + Default {}

impl SampleType for u8 {}
impl SampleType for i16 {}
impl SampleType for f32 {}

// TODO make it const-generic over channels once it lands
#[derive(Debug)]
pub struct AudioQueue<S: SampleType> {
    queue: Vec<VecDeque<S>>,
}

impl<S: SampleType> AudioQueue<S> {
    pub fn new(channels: usize) -> Self {
        AudioQueue {
            queue: vec![VecDeque::new(); channels],
        }
    }

    pub fn send(&mut self, buf: &[&[S]]) {
        assert_eq!(buf.len(), self.queue.len());
        for (b, mut q) in buf.iter().zip(&mut self.queue) {
            q.extend(b.iter());
        }
    }

    pub fn receive(&mut self, buf: &mut [&mut [S]]) {
        assert_eq!(buf.len(), self.queue.len());
        for (mut b, q) in buf.iter_mut().zip(&mut self.queue) {
            for d in b.iter_mut() {
                *d = q.pop_front().unwrap_or_default();
            }
        }
    }

    pub fn len(&self) -> usize {
        self.queue[0].len()
    }

    pub fn receive_interleaved(&mut self, buf: &mut [S]) {
        for mut ch in buf.chunks_mut(self.queue.len()) {
            for (mut b, q) in ch.iter_mut().zip(self.queue.iter_mut()) {
                *b = q.pop_front().unwrap_or_default();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const IN: &[&[u8]] = &[
        &[1, 2, 3, 4, 5],
        &[11, 12, 13, 14, 15],
        &[21, 22, 23, 24, 25],
        &[31, 32, 33, 34, 35],
    ];

    #[test]
    fn audio_queue() {
        let mut aq = AudioQueue::new(4);
        aq.send(IN);

        println!("{:?}", aq);

        let out: &mut [&mut [u8]] = &mut [&mut [0, 0], &mut [0, 0], &mut [0, 0], &mut [0, 0]];

        aq.receive(out);

        println!("{:?} {:?}", aq, out);

        assert_eq!(out, &[&[1, 2], &[11, 12], &[21, 22], &[31, 32]]);

        aq.receive(out);

        assert_eq!(out, &[&[3, 4], &[13, 14], &[23, 24], &[33, 34]]);

        aq.receive(out);

        assert_eq!(out, &[&[5, 0], &[15, 0], &[25, 0], &[35, 0]]);
    }

    #[test]
    fn audio_queue_interleave() {
        let mut aq = AudioQueue::new(4);
        let out: &mut [u8] = &mut [0; 8];
        aq.send(IN);

        println!("{:?}", aq);

        aq.receive_interleaved(out);

        assert_eq!(out, &[1, 11, 21, 31, 2, 12, 22, 32]);
    }
}
