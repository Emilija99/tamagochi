const accounts = [
  {
    name: 'a',
    address: 'secret1dm9s4w2usut2taeds3662azj5wl59ty8zjgfx9',
    mnemonic: 'all glad vessel boat begin identify inject scale mystery cigar danger exact negative nothing sweet planet praise siege pull surface prize inspire shed patrol'
  },
  {
    name: 'b',
    address: 'secret1897wspmlwk8k2vh8cppezucef3ld0grwmmymr2',
    mnemonic: 'spray pool edit can wage scan noodle welcome disease afraid skull mushroom ship harvest pepper venture tape eye parade maid buddy later slab advance'
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
