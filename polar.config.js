const accounts = [
  {
    name: 'a',
    address: 'secret1wws59d93znpn6jauza4vrmmx26q5gq40n3xr2c',
    mnemonic: 'person analyst bottom lawsuit brand plunge state upset gaze crater swing pear street praise awesome group worry plate enrich envelope zone ship such fame'
  },
  {
    name: 'b',
    address: 'secret16dxm8f3qkk8x2hlg3mpr8s5c8ccq9em6hdevqw',
    mnemonic: 'either depth rule margin wagon year clog slight salute liquid whip actual recipe fringe cheap april icon episode truth topic skate need strong size'
  }
];

const networks = {
  default: {
    endpoint: "http://192.168.1.95:1337/"
  },
  localnet: {
    endpoint: 'http://192.168.1.95:1337/',
    accounts: accounts,
  },
  development: {
    endpoint: 'tcp://0.0.0.0:26656',
    chainId: 'secretdev-1',
    types: {}
  },
 
};

module.exports = {
  networks: {
    default: networks.localnet,
    localnet: networks.localnet,
    development: networks.development
  },
  mocha: {
    timeout: 60000
  },
  rust: {
    version: "1.55.0",
  }
};
