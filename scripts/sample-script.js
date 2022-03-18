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

async function instantiate_contract(contract_owner,contract,snip_addr,snip_hash,label){

 const contract_info = await contract.instantiate({snip_addr,snip_hash}, label, contract_owner);
 console.log(contract_info);
}


async function run () {
  const contract_owner = getAccountByName("a");

  //market contract
  const contract1=new Contract("market");
  await deploy_contract(contract_owner,contract1);
  await instantiate_contract(contract_owner,contract1,"secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg","E47144CD74E2E3E24275962CAA7719F081CCFA81A46532812596CA3D5BA6ECEB","contract1");

  //pet contract

  const contract2=new Contract("pet");
  await deploy_contract(contract_owner,contract2);
  await instantiate_contract(contract_owner,contract2,"secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg","E47144CD74E2E3E24275962CAA7719F081CCFA81A46532812596CA3D5BA6ECEB","contract2")

  // use below line if contract initiation done using another contract
  // const contract_addr = "secret76597235472354792347952394";
  // contract.instantiatedWithAddress(contract_addr);
 
  /*const inc_response = await contract.tx.increment({account: contract_owner});
  console.log(inc_response);
  // to get logs as a key:value object
  // console.log(getLogs(inc_response));

  const response = await contract.query.get_count();
  console.log(response);

  const transferAmount = [{"denom": "uscrt", "amount": "15000000"}] // 15 SCRT
  const customFees = { // custom fees
    amount: [{ amount: "750000", denom: "uscrt" }],
    gas: "3000000",
  }
  const ex_response = await contract.tx.increment(
    {account: contract_owner, transferAmount: transferAmount}
  );
  // const ex_response = await contract.tx.increment(
  //   {account: contract_owner, transferAmount: transferAmount, customFees: customFees}
  // );
  console.log(ex_response);*/
}

module.exports = { default: run };
