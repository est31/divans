use ::brotli;
use ::interface::{DivansResult, DivansOutputResult};
use ::brotli::enc::interface::LiteralPredictionModeNibble;
use core;
use ::interface::{DivansCompressorOptions, BrotliCompressionSetting, StrideSelection, DivansCompressorFactory, Compressor};
use ::probability::Speed;
use super::alloc_util::SubclassableAllocator;
use super::interface::*;
type BrotliFactory = ::BrotliDivansHybridCompressorFactory<SubclassableAllocator<u8>,
                                                         SubclassableAllocator<u16>,
                                                         SubclassableAllocator<u32>,
                                                         SubclassableAllocator<i32>,
                                                         SubclassableAllocator<u64>,
                                                         SubclassableAllocator<brotli::enc::command::Command>,
                                                         SubclassableAllocator<::DefaultCDF16>,
                                                         SubclassableAllocator<brotli::enc::util::floatX>,
                                                         SubclassableAllocator<brotli::enc::vectorization::Mem256f>,
                                                         SubclassableAllocator<brotli::enc::PDF>,
                                                         SubclassableAllocator<brotli::enc::StaticCommand>,
                                                         SubclassableAllocator<brotli::enc::histogram::HistogramLiteral>,
                                                         SubclassableAllocator<brotli::enc::histogram::HistogramCommand>,
                                                         SubclassableAllocator<brotli::enc::histogram::HistogramDistance>,
                                                         SubclassableAllocator<brotli::enc::cluster::HistogramPair>,
                                                         SubclassableAllocator<brotli::enc::histogram::ContextType>,
                                                         SubclassableAllocator<brotli::enc::entropy_encode::HuffmanTree>,
                                                         SubclassableAllocator<brotli::enc::ZopfliNode>>;
type InternalCompressorFactory = ::DivansCompressorFactoryStruct<SubclassableAllocator<u8>,
                                                         SubclassableAllocator<::DefaultCDF16>>;
pub enum CompressorState {
    OptionStage(DivansCompressorOptions),
    BrotliCompressor(::BrotliDivansHybridCompressor<::DefaultCDF16,
                                                         <BrotliFactory as ::DivansCompressorFactory<SubclassableAllocator<u8>,
                                                                                                          SubclassableAllocator<u32>,
                                                                                                          SubclassableAllocator<::DefaultCDF16>>
                                                          >::DefaultEncoder,
                                                         SubclassableAllocator<u8>,
                                                         SubclassableAllocator<u16>,
                                                         SubclassableAllocator<u32>,
                                                         SubclassableAllocator<i32>,
                                                         SubclassableAllocator<u64>,
                                                         SubclassableAllocator<brotli::enc::command::Command>,
                                                         SubclassableAllocator<::DefaultCDF16>,
                                                         SubclassableAllocator<brotli::enc::util::floatX>,
                                                         SubclassableAllocator<brotli::enc::vectorization::Mem256f>,
                                                         SubclassableAllocator<brotli::enc::PDF>,
                                                         SubclassableAllocator<brotli::enc::StaticCommand>,
                                                         SubclassableAllocator<brotli::enc::histogram::HistogramLiteral>,
                                                         SubclassableAllocator<brotli::enc::histogram::HistogramCommand>,
                                                         SubclassableAllocator<brotli::enc::histogram::HistogramDistance>,
                                                         SubclassableAllocator<brotli::enc::cluster::HistogramPair>,
                                                         SubclassableAllocator<brotli::enc::histogram::ContextType>,
                                                         SubclassableAllocator<brotli::enc::entropy_encode::HuffmanTree>,
                                                         SubclassableAllocator<brotli::enc::ZopfliNode>>),
    InternalCompressor(::DivansCompressor<<InternalCompressorFactory as ::DivansCompressorFactory<SubclassableAllocator<u8>,
                                                                                                             SubclassableAllocator<u32>,
                                                                                                             SubclassableAllocator<::DefaultCDF16>>
                                               >::DefaultEncoder,
                       SubclassableAllocator<u8>,
                       SubclassableAllocator<u32>,
                       SubclassableAllocator<::DefaultCDF16>>),
}

impl Default for CompressorState {
    fn default() -> Self {
        CompressorState::OptionStage(DivansCompressorOptions::default())
    }
}
impl CompressorState {
    pub fn set_option(&mut self, selector: super::interface::DivansOptionSelect, value: u32) -> super::interface::DivansReturnCode {
        if let CompressorState::OptionStage(ref mut opts) = *self {
            match selector {
                DIVANS_OPTION_QUALITY => {opts.quality = Some(value as u16);},
                DIVANS_OPTION_WINDOW_SIZE => {opts.window_size = Some(value as i32);},
                DIVANS_OPTION_LGBLOCK => {opts.lgblock = Some(value);},
                DIVANS_OPTION_STRIDE_DETECTION_QUALITY => {opts.stride_detection_quality = Some(value as u8);},
                DIVANS_OPTION_PRIOR_BITMASK_DETECTION => {opts.prior_bitmask_detection = value as u8},
                DIVANS_OPTION_SPEED_DETECTION_QUALITY => {opts.speed_detection_quality = Some(value as u8);},
                DIVANS_OPTION_DYNAMIC_CONTEXT_MIXING => {opts.dynamic_context_mixing = Some(value as u8);},
                DIVANS_OPTION_USE_BROTLI_COMMAND_SELECTION => {opts.use_brotli = match value {
                    0 => BrotliCompressionSetting::UseInternalCommandSelection,
                    1 => BrotliCompressionSetting::UseBrotliCommandSelection,
                    2 => BrotliCompressionSetting::UseBrotliBitstream,
                    _ => return DIVANS_FAILURE,
                };},
                DIVANS_OPTION_USE_BROTLI_BITSTREAM => {opts.use_brotli = match value {
                    1 => BrotliCompressionSetting::UseBrotliBitstream,
                    _ => return DIVANS_FAILURE,
                };},
                DIVANS_OPTION_BROTLI_LITERAL_BYTE_SCORE => opts.brotli_literal_byte_score = Some(value),
                DIVANS_OPTION_USE_CONTEXT_MAP => {opts.use_context_map = match value {
                    1 => true,
                    0 => false,
                    _ => return DIVANS_FAILURE,
                };},
                DIVANS_OPTION_FORCE_STRIDE_VALUE => {opts.force_stride_value = match value {
                    0 => StrideSelection::PriorDisabled,
                    1 => StrideSelection::Stride1,
                    2 => StrideSelection::Stride2,
                    3 => StrideSelection::Stride3,
                    4 => StrideSelection::Stride4,
                    5 => StrideSelection::Stride5,
                    6 => StrideSelection::Stride6,
                    7 => StrideSelection::Stride7,
                    8 => StrideSelection::Stride8,
                    _ => return DIVANS_FAILURE,
                };},
                DIVANS_OPTION_LITERAL_ADAPTATION_STRIDE_HIGH => {
                    if value as usize >= Speed::ENCODER_DEFAULT_PALETTE.len() {
                        return DIVANS_FAILURE;
                    }
                    let literal_adaptation = Speed::ENCODER_DEFAULT_PALETTE[value as usize];
                    match opts.literal_adaptation {
                        None => opts.literal_adaptation = Some([literal_adaptation, literal_adaptation, literal_adaptation, literal_adaptation]),
                        Some(ref mut adapt) => {
                           (*adapt)[1] = literal_adaptation;
                        },
                    }
                },
                DIVANS_OPTION_LITERAL_ADAPTATION_CM_HIGH => {
                    if value as usize >= Speed::ENCODER_DEFAULT_PALETTE.len() {
                        return DIVANS_FAILURE;
                    }
                    let literal_adaptation = Speed::ENCODER_DEFAULT_PALETTE[value as usize];
                    match opts.literal_adaptation {
                        None => opts.literal_adaptation = Some([literal_adaptation, literal_adaptation, literal_adaptation, literal_adaptation]),
                        Some(ref mut adapt) => {
                           (*adapt)[3] = literal_adaptation;
                        },
                    }
                },
                DIVANS_OPTION_LITERAL_ADAPTATION_STRIDE_LOW => {
                    if value as usize >= Speed::ENCODER_DEFAULT_PALETTE.len() {
                        return DIVANS_FAILURE;
                    }
                    let literal_adaptation = Speed::ENCODER_DEFAULT_PALETTE[value as usize];
                    match opts.literal_adaptation {
                        None => opts.literal_adaptation = Some([literal_adaptation, literal_adaptation, literal_adaptation, literal_adaptation]),
                        Some(ref mut adapt) => {
                           (*adapt)[0] = literal_adaptation;
                        },
                    }
                },
                DIVANS_OPTION_LITERAL_ADAPTATION_CM_LOW => {
                    if value as usize >= Speed::ENCODER_DEFAULT_PALETTE.len() {
                        return DIVANS_FAILURE;
                    }
                    let literal_adaptation = Speed::ENCODER_DEFAULT_PALETTE[value as usize];
                    match opts.literal_adaptation {
                        None => opts.literal_adaptation = Some([literal_adaptation, literal_adaptation, literal_adaptation, literal_adaptation]),
                        Some(ref mut adapt) => {
                           (*adapt)[2] = literal_adaptation;
                        },
                    }
                },
                DIVANS_OPTION_PRIOR_DEPTH => {
                    opts.prior_depth = Some(value as u8);
                },
                DIVANS_OPTION_Q9_5 => {
                    opts.q9_5 = value as u8 != 0;
                },
                DIVANS_OPTION_FORCE_LITERAL_CONTEXT_MODE => {
                    opts.force_literal_context_mode = Some(LiteralPredictionModeNibble(value as u8));
                },
                DIVANS_OPTION_IR_OPTIMIZER => {
                    opts.divans_ir_optimizer = value as u8;
                },
                _ => return DIVANS_FAILURE,
            }
            return DIVANS_SUCCESS;
        }
        DIVANS_FAILURE
    }
    fn start(&mut self, allocators: &CAllocator, opts:DivansCompressorOptions) {
        match opts.use_brotli {
            BrotliCompressionSetting::UseInternalCommandSelection => {
                core::mem::replace(self,
                                   CompressorState::InternalCompressor(
                                       InternalCompressorFactory::new(
                                           SubclassableAllocator::<u8>::new(allocators.clone()),
                                           SubclassableAllocator::<u32>::new(allocators.clone()),
                                           SubclassableAllocator::<::DefaultCDF16>::new(allocators.clone()),
                                           opts,
                                           ())));
            },
            _ => {
                core::mem::replace(self,
                                   CompressorState::BrotliCompressor(
                                       BrotliFactory::new(
                                           SubclassableAllocator::<u8>::new(allocators.clone()),
                                           SubclassableAllocator::<u32>::new(allocators.clone()),
                                           SubclassableAllocator::<::DefaultCDF16>::new(allocators.clone()),
                                           opts,
                                           (
                                               SubclassableAllocator::<u8>::new(allocators.clone()),
                                               SubclassableAllocator::<u16>::new(allocators.clone()),
                                               SubclassableAllocator::<i32>::new(allocators.clone()),
                                               SubclassableAllocator::<brotli::enc::command::Command>::new(allocators.clone()),
                                               SubclassableAllocator::<u64>::new(allocators.clone()),
                                               SubclassableAllocator::<brotli::enc::util::floatX>::new(allocators.clone()),
                                               SubclassableAllocator::<brotli::enc::vectorization::Mem256f>::new(allocators.clone()),
                                               SubclassableAllocator::<brotli::enc::histogram::HistogramLiteral>::new(allocators.clone()),
                                               SubclassableAllocator::<brotli::enc::histogram::HistogramCommand>::new(allocators.clone()),
                                               SubclassableAllocator::<brotli::enc::histogram::HistogramDistance>::new(allocators.clone()),
                                               SubclassableAllocator::<brotli::enc::cluster::HistogramPair>::new(allocators.clone()),
                                               SubclassableAllocator::<brotli::enc::histogram::ContextType>::new(allocators.clone()),
                                               SubclassableAllocator::<brotli::enc::entropy_encode::HuffmanTree>::new(allocators.clone()),
                                               SubclassableAllocator::<brotli::enc::ZopfliNode>::new(allocators.clone()),
                                               SubclassableAllocator::<brotli::enc::PDF>::new(allocators.clone()),
                                               SubclassableAllocator::<brotli::enc::StaticCommand>::new(allocators.clone()),
                                           ))));
            
            }

        }
    }
    pub fn encode(&mut self,
              input_buf:&[u8],
              input_offset: &mut usize,
              output_buf:&mut[u8],
              output_offset: &mut usize,
              allocators: &CAllocator) -> DivansReturnCode {
        if let CompressorState::OptionStage(opts) = *self {
            self.start(allocators, opts);
        }
        let res = match *self {
            CompressorState::OptionStage(_) => unreachable!(),
            CompressorState::BrotliCompressor(ref mut compressor) => {
                compressor.encode(input_buf, input_offset, output_buf, output_offset)
            },
            CompressorState::InternalCompressor(ref mut compressor) => {
                compressor.encode(input_buf, input_offset, output_buf, output_offset)
            },
        };
        match res {
            DivansResult::Success => DIVANS_SUCCESS,
            DivansResult::Failure(_) => DIVANS_FAILURE,
            DivansResult::NeedsMoreInput => DIVANS_NEEDS_MORE_INPUT,
            DivansResult::NeedsMoreOutput => DIVANS_NEEDS_MORE_OUTPUT,
        }
    }
    pub fn flush(&mut self,
              output_buf:&mut[u8],
             output_offset: &mut usize,
             allocators: &CAllocator) -> DivansReturnCode {
        if let CompressorState::OptionStage(opts) = *self {
            self.start(allocators, opts);
        }
        let res = match *self {
            CompressorState::OptionStage(_) => unreachable!(),
            CompressorState::BrotliCompressor(ref mut compressor) => {
                compressor.flush(output_buf, output_offset)
            },
            CompressorState::InternalCompressor(ref mut compressor) => {
                compressor.flush(output_buf, output_offset)
            },
        };
        match res {
            DivansOutputResult::Success => DIVANS_SUCCESS,
            DivansOutputResult::Failure(_) => DIVANS_FAILURE,
            DivansOutputResult::NeedsMoreOutput => DIVANS_NEEDS_MORE_OUTPUT,
        }
    }
}


#[repr(C)]
#[no_mangle]
pub struct DivansCompressorState {
    pub custom_allocator: CAllocator,
    pub compressor: CompressorState
}

impl Drop for DivansCompressorState {
    fn drop(&mut self) {
        match self.compressor {
            CompressorState::OptionStage(_) => {},
            CompressorState::BrotliCompressor(ref mut compressor) => {
                compressor.free_ref();
              
            },
            CompressorState::InternalCompressor(ref mut compressor) => {
                compressor.free_ref();
            }
        }
    }
}
