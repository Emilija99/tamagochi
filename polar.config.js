const accounts = [
  {
    name: 'a',
    address: 'secret15xhwzx7ulc3tuwjsskapulxv8ggc33vqavqtyj',
    mnemonic: 'boost dune cousin key nephew crop echo puzzle violin spike divert firm neck unit enemy category unique universe guitar volcano river enrich auction drum'
  },
  {
    name: 'b',
    address: 'secret1ye0shqg9s2tu2764v3d5t5mpl52acvtgevhag4',
    mnemonic: 'dumb fox raw bulb sauce divert upgrade step february average obey attack hidden grid fall onion bird indoor skate salon help inflict grid sausage'
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
