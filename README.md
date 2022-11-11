

           / /\/ / \ \/\_/ \|
      ___/                   ---
      \   ###   ##  #   #  #   /
      -   #  # #  # # # #  #   -
      \   ###  #  # ## ##       \
      /   #     ##  #   #  #   --
      --                     /
         \/\   /\ / \ | \/ /



# Usage

Edit the included `example-config.json` to suit your needs (pool address,
number of threads, etc), then run:

`powhasher -c your-config.json`

While the hasher is running, press Enter to get statistics.

# What is it?

This is a simple CLI miner for modern x86 CPUs, powered by the
[yellowsun](https://github.com/kazcw/yellowsun) CryptoNight hash implementation
and the [cn-stratum](https://github.com/kazcw/cn-stratum) pool client. It once
hosted unusually fast Cn/Cnv1 c