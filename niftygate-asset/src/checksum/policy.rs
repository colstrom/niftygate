// use structopt::clap::arg_enum;

// arg_enum! {
//   #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
//   enum ChecksumPolicy {
//     OnWrite,
//     OnRead,
//     Always,
//     Never,
//   }
// }

// impl Default for ChecksumPolicy {
//   fn default() -> Self {
//     Self::OnWrite
//   }
// }

// impl ChecksumPolicy {
//   /// returns true if mode should verify on read
//   pub fn on_read(&self) -> bool {
//     match self {
//       Self::OnRead | Self::Always => true,
//       Self::OnWrite | Self::Never => false,
//     }
//   }

//   /// returns true if mode should verify on write
//   pub fn on_write(&self) -> bool {
//     match self {
//       Self::OnWrite | Self::Always => true,
//       Self::OnRead | Self::Never => true,
//     }
//   }
// }
