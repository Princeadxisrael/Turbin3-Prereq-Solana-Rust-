mod programs;

#[cfg(test)] 
mod tests{ 
use solana_program::system_program;
use solana_client::rpc_client::RpcClient; 
use solana_program::{ pubkey::Pubkey, system_instruction::transfer, };
 use solana_sdk::{message::Message, signature::{Keypair, Signer, read_keypair_file}, transaction:: Transaction};
 use bs58; 
use std::io::{self, BufRead};
use std::str::FromStr;
use crate::programs::wba_prereq::{WbaPrereqProgram, CompleteArgs};

//Establish a connection to the devnet cluster
const RPC_URL: &str = "https://api.devnet.solana.com";


//annotating with tests
    #[test]
    fn keygen() {
        //prints a new keypair 
        let kp = Keypair::new(); 
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string()); 
        println!(""); 
        println!("To save your wallet, copy and paste the following into a JSON file:"); 
        println!("{:?}", kp.to_bytes()); 
    } 
    #[test] 
    fn airdop() {
        //executes an airdrop
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        // Connected to Solana Devnet RPC Client 
        let client = RpcClient::new(RPC_URL);
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) { 
            Ok(s) => { 
                println!("Success! Check out your TX here:"); 
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string()); 
            }, 
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()) 
        };
    } 
    #[test] 
    fn transfer_sol() {
        //initiate a transfer
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        //recipient public key
        let to_pubkey= Pubkey::from_str("A3hrWWfodxQgG8LB3VD5Bd32RNGSe6krdV43EhPojLy9").unwrap();

        //establish a connection
        let rpc_client= RpcClient::new(RPC_URL);

        // Get recent blockhash 
        let recent_blockhash = rpc_client .get_latest_blockhash() .expect("Failed to get recent blockhash");

        //sign transaction
        let transaction= Transaction::new_signed_with_payer(&[transfer(
            &keypair.pubkey(),
            &to_pubkey,
            1_000_000
        )], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash);

        //send the transaction
        let signature= rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");
        print!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature
        );

        // Get the exact balance of the account 
        // Calculate the fee of sending the transaction 
        // Calculate the exact number of lamports we can send whilst satisfying the fee rate 
        // Start by adding Message to our imports.

        let balance= rpc_client.get_balance(&keypair.pubkey()).expect("Failed to get balance");

        // Create a test transaction to calculate fees
        let message=Message::new_with_blockhash(&[transfer(&keypair.pubkey(),&to_pubkey, balance)], Some(&keypair.pubkey()), &recent_blockhash);

        // Calculate exact fee rate to transfer entire SOL amount out of account minus fees
        let fee=rpc_client.get_fee_for_message(&message). expect("failed to get fee caculator");

        let _transaction= Transaction::new_signed_with_payer(&[transfer(
            &keypair.pubkey(),
            &to_pubkey,
            balance-fee
        )], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash);

     
    }
    #[test]
    fn base58_to_wallet(){
        println!("Input your private key as base58:"); 
        let stdin = io::stdin(); 
        let base58 = stdin.lock().lines().next().unwrap().unwrap(); 
        println!("Your wallet file is:"); 
        let wallet = bs58::decode(base58).into_vec().unwrap(); 
        println!("{:?}", wallet); 
    }
    #[test]
    fn wallet_to_base58(){
        println!("Input your private key as a wallet file byte array:"); 
        let stdin = io::stdin(); 
        let wallet = stdin.lock().lines().next().unwrap().unwrap().trim_start_matches('[').trim_end_matches(']').split(',') 
        .map(|s| s.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>(); 
        println!("Your private key is:"); 
        let base58 = bs58::encode(wallet).into_string(); 
        println!("{:?}", base58); 
    }  
    #[test] 
    fn consume_idl(){
        // Create a Solana devnet connection 
        let rpc_client = RpcClient::new(RPC_URL);

        // define accounts 
        let signer = read_keypair_file("wba-wallet.json").expect("Couldn't find wallet file");


        let prereq = WbaPrereqProgram::derive_program_address(&[b"prereq", 
        signer.pubkey().to_bytes().as_ref()]);

        let args = CompleteArgs { github: b"princeadxisrael".to_vec() };

        // Get recent blockhash
         let blockhash = rpc_client .get_latest_blockhash().expect("Failed to get recent blockhash");

        //Invoke the "complete" function 
        let transaction = WbaPrereqProgram::complete( 
            &[&signer.pubkey(), &prereq, &system_program::id()], 
            &args, Some(&signer.pubkey()), 
            &[&signer], 
            blockhash );

            // Send the transaction
            let signature = rpc_client .send_and_confirm_transaction(&transaction) 
            .expect("Failed to send transaction"); 
        // Print our transaction out 
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);


    }
}


