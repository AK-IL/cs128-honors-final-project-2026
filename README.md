# Encrypted Chat w/ Panic Mode
## Group Name  
ATA

## Group member names and NetIDs
Aekam Singh Pal \- aekam2  
Thomas Judge \- tjudge2  
Aditya Khanna \- akhanna7

## Project Introduction  
Simple chat server and client utilizing double-rachet encryption. Should support group chats, images, links, and potentially other features. Panic mode that replaces message history with fake messages.

## Technical Overview  
Clients will coordinate with the server to share public keys (utilizing Diffie–Hellman key exchange). Messages will be end to end encrypted using MLS or the Signal protocol. Panic mode will be implemented using pregenerated random messages (saved in JSON or other format) for speed and offline support.

### Checkpoint 1:
- Rust server setup (using tokio) to handle users and to route messages  
- Set up the Diffie-Hellman key exchange  
- Create authentication system that accepts different passwords (for regular or panic mode)  
- Message basics (Standard text and group chat routing working)

### Checkpoint 2:
- Signal / MLS Encryption Setup  
- Panic Mode integration  
- Ensure group keys work to not compromise past messages

    
## Possible Challenges    
- Managing users safety  
- Using encryption libraries correctly   
  - Exchanging public keys without leaking private keys  
- Bugs that happen for users might be hard to reproduce and understand   
- Making the structure of everything look nice, time, and names  
- Figuring out what services are needed and if we are going to overkill on some things
