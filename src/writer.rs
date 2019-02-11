#[cfg(feature="std")]
use std::io::{self, Error, ErrorKind, Write};
#[cfg(feature="std")]
pub use alloc_stdlib::StandardAlloc;
#[cfg(all(feature="unsafe",feature="std"))]
pub use alloc_stdlib::HeapAllocUninitialized;
pub use huffman::{HuffmanCode, HuffmanTreeGroup};
pub use state::BrotliState;
// use io_wrappers::write_all;
pub use io_wrappers::{CustomWrite};
#[cfg(feature="std")]
pub use io_wrappers::{IntoIoWriter, IoWriterWrapper};
pub use super::decode::{BrotliDecompressStream, BrotliResult};
pub use alloc::{AllocatedStackMemory, Allocator, SliceWrapper, SliceWrapperMut, StackAllocator};

#[cfg(feature="std")]
pub struct DecompressorWriterCustomAlloc<W: Write,
     BufferType : SliceWrapperMut<u8>,
     AllocU8 : Allocator<u8>,
     AllocU32 : Allocator<u32>,
     AllocHC : Allocator<HuffmanCode> >(DecompressorWriterCustomIo<io::Error,
                                                             IntoIoWriter<W>,
                                                             BufferType,
                                                             AllocU8, AllocU32, AllocHC>);


#[cfg(feature="std")]
impl<W: Write,
     BufferType : SliceWrapperMut<u8>,
     AllocU8,
     AllocU32,
     AllocHC> DecompressorWriterCustomAlloc<W, BufferType, AllocU8, AllocU32, AllocHC>
 where AllocU8 : Allocator<u8>, AllocU32 : Allocator<u32>, AllocHC : Allocator<HuffmanCode>
    {
    pub fn new(w: W, buffer : BufferType,
               alloc_u8 : AllocU8, alloc_u32 : AllocU32, alloc_hc : AllocHC) -> Self {
     let dict = AllocU8::AllocatedMemory::default();
     Self::new_with_custom_dictionary(w, buffer, alloc_u8, alloc_u32, alloc_hc, dict)

    }
    pub fn new_with_custom_dictionary(w: W, buffer : BufferType,
               alloc_u8 : AllocU8, alloc_u32 : AllocU32, alloc_hc : AllocHC, dict: AllocU8::AllocatedMemory) -> Self {
        DecompressorWriterCustomAlloc::<W, BufferType, AllocU8, AllocU32, AllocHC>(
          DecompressorWriterCustomIo::<Error,
                                 IntoIoWriter<W>,
                                 BufferType,
                                 AllocU8, AllocU32, AllocHC>::new_with_custom_dictionary(IntoIoWriter::<W>(w),
                                                                  buffer,
                                                                  alloc_u8, alloc_u32, alloc_hc,
                                                                  dict,
                                                                  Error::new(ErrorKind::InvalidData,
                                                                             "Invalid Data")))
    }

    pub fn get_ref(&self) -> &W {
        &self.0.get_ref().0
    }
    pub fn get_mut(&mut self) -> &mut W {
        &mut self.0.get_mut().0
    }
    pub fn into_inner(self) -> W {
        self.0.into_inner().0
    }
}
#[cfg(feature="std")]
impl<W: Write,
     BufferType : SliceWrapperMut<u8>,
     AllocU8 : Allocator<u8>,
     AllocU32 : Allocator<u32>,
     AllocHC : Allocator<HuffmanCode> > Write for DecompressorWriterCustomAlloc<W,
                                                                         BufferType,
                                                                         AllocU8,
                                                                         AllocU32,
                                                                         AllocHC> {
  	fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
       self.0.write(buf)
    }
  	fn flush(&mut self) -> Result<(), Error> {
       self.0.flush()
    }
}


#[cfg(not(any(feature="unsafe", not(feature="std"))))]
pub struct DecompressorWriter<W: Write>(DecompressorWriterCustomAlloc<W,
                                                         <StandardAlloc
                                                          as Allocator<u8>>::AllocatedMemory,
                                                         StandardAlloc,
                                                         StandardAlloc,
                                                         StandardAlloc>);


#[cfg(not(any(feature="unsafe", not(feature="std"))))]
impl<W: Write> DecompressorWriter<W> {
  pub fn new(w: W, buffer_size: usize) -> Self {
      Self::new_with_custom_dictionary(w, buffer_size, <StandardAlloc as Allocator<u8>>::AllocatedMemory::default())
  }
  pub fn new_with_custom_dictionary(w: W, buffer_size: usize, dict: <StandardAlloc as Allocator<u8>>::AllocatedMemory) -> Self {
    let mut alloc = StandardAlloc::default();
    let buffer = <StandardAlloc as Allocator<u8>>::alloc_cell(&mut alloc, if buffer_size == 0 {4096} else {buffer_size});
    DecompressorWriter::<W>(DecompressorWriterCustomAlloc::<W,
                                                <StandardAlloc
                                                 as Allocator<u8>>::AllocatedMemory,
                                                StandardAlloc,
                                                StandardAlloc,
                                                StandardAlloc>::new_with_custom_dictionary(w,
                                                                              buffer,
                                                                              alloc,
                                                                              StandardAlloc::default(),
                                                                              StandardAlloc::default(),
                                                                              dict))
  }

  pub fn get_ref(&self) -> &W {
      self.0.get_ref()
  }
  pub fn get_mut(&mut self) -> &mut W {
      self.0.get_mut()
  }
  pub fn into_inner(self) -> W {
    self.0.into_inner()
  }
}


#[cfg(all(feature="unsafe", feature="std"))]
pub struct DecompressorWriter<W: Write>(DecompressorWriterCustomAlloc<W,
                                                         <HeapAllocUninitialized<u8>
                                                          as Allocator<u8>>::AllocatedMemory,
                                                         HeapAllocUninitialized<u8>,
                                                         HeapAllocUninitialized<u32>,
                                                         HeapAllocUninitialized<HuffmanCode> >);


#[cfg(all(feature="unsafe", feature="std"))]
impl<W: Write> DecompressorWriter<W> {
  pub fn new(w: W, buffer_size: usize) -> Self {
    let dict = <HeapAllocUninitialized<u8> as Allocator<u8>>::AllocatedMemory::default();
    Self::new_with_custom_dictionary(w, buffer_size, dict)
  }
  pub fn new_with_custom_dictionary(w: W, buffer_size: usize, dict: <HeapAllocUninitialized<u8> as Allocator<u8>>::AllocatedMemory) -> Self {
    let mut alloc_u8 = unsafe { HeapAllocUninitialized::<u8>::new() };
    let buffer = alloc_u8.alloc_cell(buffer_size);
    let alloc_u32 = unsafe { HeapAllocUninitialized::<u32>::new() };
    let alloc_hc = unsafe { HeapAllocUninitialized::<HuffmanCode>::new() };
    DecompressorWriter::<W>(DecompressorWriterCustomAlloc::<W,
                                                <HeapAllocUninitialized<u8>
                                                 as Allocator<u8>>::AllocatedMemory,
                                                HeapAllocUninitialized<u8>,
                                                HeapAllocUninitialized<u32>,
                                                HeapAllocUninitialized<HuffmanCode> >
      ::new_with_custom_dictionary(w, buffer, alloc_u8, alloc_u32, alloc_hc, dict))
  }

  pub fn get_ref(&self) -> &W {
      self.0.get_ref()
  }
  pub fn get_mut(&mut self) -> &mut W {
    &mut (self.0).0.get_mut().0
  }
  pub fn into_inner(self) -> W {
    self.0.into_inner().0
  }
}


#[cfg(feature="std")]
impl<W: Write> Write for DecompressorWriter<W> {
  	fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
       self.0.write(buf)
    }
  	fn flush(&mut self) -> Result<(), Error> {
       self.0.flush()
    }
}

pub struct DecompressorWriterCustomIo<ErrType,
                                W: CustomWrite<ErrType>,
                                BufferType: SliceWrapperMut<u8>,
                                AllocU8: Allocator<u8>,
                                AllocU32: Allocator<u32>,
                                AllocHC: Allocator<HuffmanCode>>
{
  output_buffer: BufferType,
  total_out: usize,
  output: Option<W>,
  error_if_invalid_data: Option<ErrType>,
  state: BrotliState<AllocU8, AllocU32, AllocHC>,
}


pub fn write_all<ErrType, W: CustomWrite<ErrType>>(writer: &mut W, mut buf : &[u8]) -> Result<(), ErrType> {
    while buf.len() != 0 {
          match writer.write(buf) {
                Ok(bytes_written) => buf = &buf[bytes_written..],
                Err(e) => return Err(e),
          }
    }
    Ok(())
}


impl<ErrType,
     W: CustomWrite<ErrType>,
     BufferType : SliceWrapperMut<u8>,
     AllocU8,
     AllocU32,
     AllocHC> DecompressorWriterCustomIo<ErrType, W, BufferType, AllocU8, AllocU32, AllocHC>
 where AllocU8 : Allocator<u8>, AllocU32 : Allocator<u32>, AllocHC : Allocator<HuffmanCode>
{

    pub fn new(w: W, buffer : BufferType,
               alloc_u8 : AllocU8, alloc_u32 : AllocU32, alloc_hc : AllocHC,
               invalid_data_error_type : ErrType) -> Self {
           let dict = AllocU8::AllocatedMemory::default();
           Self::new_with_custom_dictionary(w, buffer, alloc_u8, alloc_u32, alloc_hc, dict, invalid_data_error_type)
    }
    pub fn new_with_custom_dictionary(w: W, buffer : BufferType,
               alloc_u8 : AllocU8, alloc_u32 : AllocU32, alloc_hc : AllocHC,
               dict: AllocU8::AllocatedMemory,
               invalid_data_error_type : ErrType) -> Self {
        DecompressorWriterCustomIo::<ErrType, W, BufferType, AllocU8, AllocU32, AllocHC>{
            output_buffer : buffer,
            total_out : 0,
            output: Some(w),
            state : BrotliState::new_with_custom_dictionary(alloc_u8,
                                                                 alloc_u32,
                                                                 alloc_hc,
                                                                 dict),
            error_if_invalid_data : Some(invalid_data_error_type),
        }
    }
    fn close(&mut self) -> Result<(), ErrType>{
        loop {
            let mut avail_in : usize = 0;
            let mut input_offset : usize = 0;
            let mut avail_out : usize = self.output_buffer.slice_mut().len();
            let mut output_offset : usize = 0;
            let ret = BrotliDecompressStream(
                &mut avail_in,
                &mut input_offset,
                &[],
                &mut avail_out,
                &mut output_offset,
                self.output_buffer.slice_mut(),                
                &mut self.total_out,
                &mut self.state);
          match write_all(self.output.as_mut().unwrap(), &self.output_buffer.slice_mut()[..output_offset]) {
            Ok(_) => {},
            Err(e) => return Err(e),
           }
           match ret {
           BrotliResult::NeedsMoreInput => return Err(self.error_if_invalid_data.take().unwrap()),
           BrotliResult::NeedsMoreOutput => {},
           BrotliResult::ResultSuccess => return Ok(()),
           BrotliResult::ResultFailure => return Err(self.error_if_invalid_data.take().unwrap()),
           }
        }
    }

    pub fn get_ref(&self) -> &W {
        self.output.as_ref().unwrap()
    }
    pub fn get_mut(&mut self) -> &mut W {
        self.output.as_mut().unwrap()
    }
    pub fn into_inner(mut self) -> W {
      core::mem::replace(&mut self.output, None).unwrap()
    }
}
impl<ErrType,
     W: CustomWrite<ErrType>,
     BufferType : SliceWrapperMut<u8>,
     AllocU8 : Allocator<u8>,
     AllocU32 : Allocator<u32>,
     AllocHC : Allocator<HuffmanCode> > Drop for DecompressorWriterCustomIo<ErrType,
                                                                                     W,
                                                                                     BufferType,
                                                                                     AllocU8,
                                                                                     AllocU32,
                                                                                     AllocHC> {
  fn drop(&mut self) {
    match self.close() {
          Ok(_) => {},
          Err(_) => {},
    }
  }
}
impl<ErrType,
     W: CustomWrite<ErrType>,
     BufferType : SliceWrapperMut<u8>,
     AllocU8 : Allocator<u8>,
     AllocU32 : Allocator<u32>,
     AllocHC : Allocator<HuffmanCode> > CustomWrite<ErrType> for DecompressorWriterCustomIo<ErrType,
                                                                                     W,
                                                                                     BufferType,
                                                                                     AllocU8,
                                                                                     AllocU32,
                                                                                     AllocHC> {
	fn write(&mut self, buf: &[u8]) -> Result<usize, ErrType > {
        let mut avail_in = buf.len();
        let mut input_offset : usize = 0;
        loop {
            let mut output_offset = 0;
            let mut avail_out = self.output_buffer.slice_mut().len();
            let op_result = BrotliDecompressStream(&mut avail_in,
                                     &mut input_offset,
                                     &buf[..],
                                     &mut avail_out,
                                     &mut output_offset,
                                     self.output_buffer.slice_mut(),
                                     &mut self.total_out,
                                     &mut self.state);
         match write_all(self.output.as_mut().unwrap(), &self.output_buffer.slice_mut()[..output_offset]) {
          Ok(_) => {},
          Err(e) => return Err(e),
         }
         match op_result {
          BrotliResult::NeedsMoreInput => assert_eq!(avail_in, 0),
          BrotliResult::NeedsMoreOutput => continue,
          BrotliResult::ResultSuccess => return Ok((buf.len())),
          BrotliResult::ResultFailure => return Err(self.error_if_invalid_data.take().unwrap()),
        }
        if avail_in == 0 {
           break
        }
      }
      Ok(buf.len())
    }
    fn flush(&mut self) -> Result<(), ErrType> {
       self.output.as_mut().unwrap().flush()
    }
}

