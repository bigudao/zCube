use anchor_lang::prelude::*;
pub mod constant;
pub mod states;
use crate::{constant::*, states::*};

declare_id!("8MXA6CXhvVeViRS4PwzUxVEL9bzMvxtGbBYZN8GQx2Um");

#[program]
pub mod bigu_zcube {
    use super::*;

    pub fn initialize_user(
        ctx: Context<InitializeUser>
    ) -> Result<()> {
        // Initialize user profile with default data
  
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.last_zCube = 0;
        user_profile.zCube_count = 0;

        Ok(())
    }

    pub fn add_zCube(
        ctx: Context<Addzcube>, 
        location: String, 
        country: String, 
        price: String,
        img: String,
    ) -> Result<()> {
        let zCube_account = &mut ctx.accounts.zcube_account;
        let user_profile = &mut ctx.accounts.user_profile;

        // Fill contents with argument
        zcube_account.authority = ctx.accounts.authority.key();
        zcube_account.idx = user_profile.last_airbnb;
        zcube_account.location = location;
        zcube_account.country = country;
        zcube_account.price = price;
        zcube_account.image = img;
        zcube_account.isReserved = false;

        // Increase airbnb idx for PDA
        user_profile.last_zcube = user_profile.last_zcube
            .checked_add(1)
            .unwrap();

        // Increase total airbnb count
        user_profile.zcube_count = user_profile.zcube_count
            .checked_add(1)
            .unwrap();

        Ok(())
    }

    pub fn update_zcube(
        ctx: Context<Updatezcube>, 
        airbnb_idx: u8,
        location: String, 
        country: String, 
        price: String,
        img: String,
    ) -> Result<()> {
        let zcube_account = &mut ctx.accounts.zcube_account;

        // Mark todo
        zcube_account.location = location;
        zcube_account.country = country;
        zcube_account.price = price;
        zcube_account.image = img;
        Ok(())
    }

    pub fn remove_zcube(ctx: Context<RemovezCube>, _zcube_idx: u8) -> Result<()> {
        // Decreate total airbnb count
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.zcube_count = user_profile.zcube_count
            .checked_sub(1)
            .unwrap();

        // No need to decrease last zcube idx

        // Todo PDA already closed in context

        Ok(())
    }

    // Need a function that reserves an Airbnb
    pub fn book_zcube(
        ctx: Context<BookzCube>,
        idx: u8,
        date: String,
        location: String, 
        country: String, 
        price: String,
        img: String,
    ) -> Result<()> {
        let booking_account = &mut ctx.accounts.booking_account;
        
        // // Fill contents with argument
        booking_account.authority = ctx.accounts.authority.key();
        booking_account.idx = idx;
        booking_account.date = date;
        booking_account.location = location;
        booking_account.country = country;
        booking_account.price = price;
        booking_account.image = img;
        booking_account.isReserved = true;

        
        Ok(())
    }

    pub fn cancel_booking(ctx: Context<CancelBook>, _booking_idx: u8) -> Result<()> {
        // Decreate total airbnb count
        let user_profile = &mut ctx.accounts.user_profile;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>(),
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct Addzcube<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        init,
        seeds = [zCube_TAG, authority.key().as_ref(), &[user_profile.last_zcube]],
        bump,
        payer = authority,
        space = 2865 + 8,
    )]
    pub zcube_account: Box<Account<'info, AirbnbAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(zcube_idx: u8)]
pub struct Updatezcube<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        seeds = [zcube_TAG, authority.key().as_ref(), &[airbnb_idx].as_ref()],
        bump,
        has_one = authority,
    )]
    pub zcube_account: Box<Account<'info, zubeAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(zcube_idx: u8)]
pub struct RemovezCube<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,
 
    #[account(
        mut,
        close = authority,
        seeds = [zCube_TAG, authority.key().as_ref(), &[zCube_idx].as_ref()],
        bump,
        has_one = authority,
    )]
    pub zcube_account: Box<Account<'info, zCubeAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// #[derive(Accounts)]
// #[instruction()]
// pub struct BookZcube<'info> {
//     #[account(
//         mut,
//         seeds = [USER_TAG, authority.key().as_ref()],
//         bump,
//         has_one = authority,
//     )]
//     pub user_profile: Box<Account<'info, UserProfile>>,

//     #[account(
//         init,
//         seeds = [BOOK_TAG, zcube_account.key().as_ref()],
//         bump,
//         payer = booking_authority,
//         space = 3125 + 8,
//     )]
//     pub booking_account: Box<Account<'info, BookingAccount>>,

//     #[account(mut)]
//     pub authority: Signer<'info>,

//     pub system_program: Program<'info, System>,
// }

#[derive(Accounts)]
#[instruction()]
pub struct BookzCube<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,
    
    #[account(
        init,
        seeds = [BOOK_TAG, authority.key().as_ref() ],
        bump,
        payer = authority,
        space = 3125 + 8,
    )]
    pub booking_account: Box<Account<'info, BookingAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelBook<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,
 
    #[account(
        mut,
        close = authority,
        seeds = [BOOK_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub booking_account: Box<Account<'info, BookingAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
