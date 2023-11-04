// getUserProfile.cdc
import Profile from 0xProfile // Replace with actual contract address

pub fun main(address: Address): Profile.ReadOnly? {
    return Profile.fetchProfile(address: address) // Assuming `fetchProfile` is the correct function to get profile data
}
