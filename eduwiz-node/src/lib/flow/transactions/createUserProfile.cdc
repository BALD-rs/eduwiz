// createUserProfile.cdc
import Profile from 0xProfile // Replace with actual contract address

transaction(username: String) {
    prepare(signer: AuthAccount) {
        Profile.createProfile(signer: signer, username: username) // Assuming `createProfile` is the function
    }
}
