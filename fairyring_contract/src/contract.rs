// contract.rs
use cosmwasm_std::{attr, entry_point, to_json_binary, Binary, QueryRequest, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use prost::Message;
use crate::msg::{ExecuteContractMsg, QueryMsg, QueryResponse, InstantiateMsg, QueryDecryptDataResponse};
use crate::state::STORED_DATA;

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteContractMsg,
) -> StdResult<Response> {
    // Store the data
    
    // Check if identity is a non-empty string
    if msg.identity.trim().is_empty() {
        return Err(StdError::generic_err("Identity cannot be empty"));
    }

    // Use the identity directly
    let identity = msg.identity;

    STORED_DATA.save(
        deps.storage, 
        &identity, 
        &(msg.pubkey.clone(), msg.aggr_keyshare.clone()),  // Use `.clone()` to avoid moving the values
    )?;
    
    // Return a response
    Ok(Response::new()
        .add_attributes(vec![
            attr("action", "store_data"),
            attr("identity", identity),
            attr("pubkey", msg.pubkey),
            attr("aggr_keyshare", msg.aggr_keyshare),
        ])
    )
}


#[entry_point]
pub fn query(
    deps: Deps<QueryMsg>,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetStoredData { identity } => {
            if identity.trim().is_empty() {
                return Err(StdError::generic_err("Identity cannot be empty"));
            }
            let stored_data = STORED_DATA.load(deps.storage, &identity)?;
            let response = QueryResponse {
                pubkey: stored_data.0,
                aggr_keyshare: stored_data.1,
            };
            to_json_binary(&response)
        }

        QueryMsg::DecryptData {
            pubkey,
            aggr_keyshare,
            encrypted_data,
        } => {
            if encrypted_data.trim().is_empty() || aggr_keyshare.trim().is_empty() {
                return Err(StdError::generic_err("Invalid input data"));
            }

            // Call the function to query the `pep` module
            let response = query_pep_decrypt(deps, pubkey, aggr_keyshare, encrypted_data)?;

            // Return the decrypted data in binary format
            to_json_binary(&response)
        }
    }
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // Add logic to initialize the contract state here

    // For example, store some initial data
    if msg.identity.trim().is_empty() {
        return Err(StdError::generic_err("Identity cannot be empty"));
    }

    STORED_DATA.save(
        deps.storage,
        &msg.identity,
        &(msg.pubkey.clone(), msg.aggr_keyshare.clone()),
    )?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}


// Function to query the `DecryptData` RPC from your `pep` module
pub fn query_pep_decrypt(
    deps: Deps<QueryMsg>,
    pubkey : String,
    aggr_keyshare: String,
    encrypted_data: String,
) -> StdResult<String> {
    // Create the custom query
    let request = QueryRequest::Custom(QueryMsg::DecryptData {
        pubkey,
        aggr_keyshare,
        encrypted_data,
    });

    // Send the query
    let raw_response: Binary = deps.querier.query(&request)?;

    // Deserialize the protobuf response
    let query_response = QueryDecryptDataResponse::decode(&*raw_response.0)
        .map_err(|_| StdError::generic_err("Failed to decode protobuf response"))?;

    Ok(query_response.decrypted_data)
}