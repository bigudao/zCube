# zCube
RWA Marketplace for Booking All Kinds of Accomodation in SOLANA

-------------------------------------------------------------------------------------------------------------------------------------------------------------------------
## This Logic can be used in all existing hotel booking application as a plugin to facilitate payments on chain instead of using other API and paying 1% Transaction Fee
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------


# Systerm Design

## Admin View

<img width="1186" alt="admin-view" src="https://github.com/bigudao/zCube/assets/66848339/2dd7b182-e4b9-4825-9d83-77677d37cd88">

## User View

<img width="949" alt="user-view" src="https://github.com/bigudao/zCube/assets/66848339/0e5d3616-1bce-4223-8862-0f3e072e647d">

## Rust program using the Solana blockchain framework logic for consumer booking dApp. Let's break down the components:

1. **Libraries and Modules**: 
   - `anchor_lang::prelude::*`: It's bringing the necessary items from the Anchor framework into scope.
   - `pub mod constant;` and `pub mod states;`: These are likely modules containing constants and state definitions, respectively. They're brought into scope using `use crate::{constant::*, states::*};`.

2. **Program Declaration**:
   - `declare_id!("8MXA6CXhvVeViRS4PwzUxVEL9bzMvxtGbBYZN8GQx2Um");`: This macro is used to declare the ID for the program on the Solana blockchain.

3. **Program Definition** (`bigu_zcube`):
   - This is the main program definition using the `#[program]` attribute.
   - It contains several public functions (`initialize_user`, `add_zcube`, `update_zcube`, `remove_zcube`, `book_zcube`, and `cancel_booking`) representing actions that can be performed on the blockchain.

4. **Structs and Traits**:
   - `InitializeUser`, `Addzcube`, `Updatezcube`, `RemovezCube`, `BookzCube`, `CancelBook`: These are account structs representing different types of accounts involved in the program. Each struct is decorated with the `#[derive(Accounts)]` attribute, which is used by Anchor to generate necessary implementations.
   - Each struct contains fields representing accounts involved in the corresponding operation. These accounts include user profiles, Airbnb accounts, booking accounts, authority signers, and the system program.

5. **Function Implementation**:
   - Each function in the program corresponds to an action that can be performed on the blockchain.
   - For example, `initialize_user` initializes a user profile with default data, `add_zcube` adds a new Airbnb listing, `update_zcube` updates an existing Airbnb listing, `remove_zcube` removes an Airbnb listing, `book_zcube` books a zCube (whatever that may represent), and `cancel_booking` cancels a booking.

6. **Instructions**:
   - The `#[instruction]` attribute on the account structs specifies additional data that needs to be passed along with the transaction. For example, `Addzcube` expects a `zcube_idx` parameter.
   - These instructions help in providing additional context or parameters required for executing the transaction correctly.

Overall, this code define a Solana program for managing user profiles, zcube listings, and bookings on the blockchain. The Anchor framework is used to simplify the development of Solana programs by providing higher-level abstractions.



# Target Timelines
## Test-Net & Dev-Net Q2 2024
## Beta Testing Q2-Q3 2024
## Main-Net Q4 2024





