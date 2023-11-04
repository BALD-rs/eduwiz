import UserProfile from 0x01

transaction(name: String) {
    prepare(signer: AuthAccount) {
        // Check if a `User` resource is already stored at the specified path
        if signer.borrow<&UserProfile.User>(from: /storage/userName) != nil {
            // Remove the existing `User` resource and clean up storage
            let oldUserName <- signer.load<@UserProfile.User>(from: /storage/userName) 
                                ?? panic("Could not load the existing User resource.")
            // You can now do something with the old resource, like logging or transferring it, before destroying it.
            destroy oldUserName
        }

        // Now that the old resource is removed, you can save the new one.
        let newUserName: @UserProfile.User <- UserProfile.createUser(name: name)
        signer.save(<- newUserName, to: /storage/userName)
    }

    execute {
        // Execution logic after the resource is overwritten can be placed here.
    }
}
