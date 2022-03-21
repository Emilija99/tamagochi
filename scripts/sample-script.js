const { Contract, getAccountByName, getLogs } = require("secret-polar");

async function deploy_contract(contract_owner,contract){

  await contract.parseSchema();

 const deploy_response = await contract.deploy(
   contract_owner,
   { // custom fees
    amount: [{ amount: "750000", denom: "uscrt" }],
    gas: "3000000",
  }
 );
 console.log(deploy_response);

}

async function instantiate_contract(contract,contract_owner,init_msg,label){

 const contract_info = await contract.instantiate(init_msg, label, contract_owner);
 console.log(contract_info);
}


async function run () {
  const contract_owner = getAccountByName("a");

  //market contract
  const contract1=new Contract("market");
  
  await deploy_contract(contract_owner,contract1);
  
  const market_msg={snip_addr:"secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg",snip_hash:"E47144CD74E2E3E24275962CAA7719F081CCFA81A46532812596CA3D5BA6ECEB",pet_price:"10000"};
  await instantiate_contract(contract1,contract_owner,market_msg,"contract1");
  
  //pet contract
  const contract2=new Contract("pet");
  await deploy_contract(contract_owner,contract2);
 
 const pet_msg={snip_info:{addr:"secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg",hash:"E47144CD74E2E3E24275962CAA7719F081CCFA81A46532812596CA3D5BA6ECEB"},
 pet_info:{full_hours:1,alive_hours:4,feeding_price:"100"},
 market_addr:"secret10pyejy66429refv3g35g2t7am0was7ya6hvrzf"};
 await instantiate_contract(contract2,contract_owner,pet_msg,"contract2");
 
}

module.exports = { default: run };
