import * as fcl from '@onflow/fcl'
import './config'
import { writable } from 'svelte/store'
import { browser } from '$app/environment'
import { goto } from '$app/navigation'

export async function logIn() {
  const u = await fcl.authenticate()
  if (u.loggedIn) {
    goto('/home')
  }
}

export async function logOut() {
  await fcl.unauthenticate()
  goto('/')
}

export async function getAccount() {
  const address = (await fcl.currentUser.snapshot()).addr;
  return (await fcl.send([await fcl.getAccount(address)])).account;
}

export const user = writable({
  loggedIn: false,
  addr: null,
  username: null,
})

if (browser) {
  fcl.currentUser.subscribe(user.set)
}

export async function getUsername() {
  const address = (await fcl.currentUser.snapshot()).addr;
  const response = await fcl.query({
    cadence:`
    import UserProfile from 0x6e6efd2c0e2ad3c3

    pub fun main(user: Address): String {
      let auth: AuthAccount = getAuthAccount(user)
      let nameRef: &UserProfile.User = auth.borrow<&UserProfile.User>(from: /storage/userName) ?? panic("This User Does Not Exist!")
      return nameRef.name
    }    
      `,
      args: (arg, t) => [arg(address, t.Address)], // Adding the required argument
  })
  console.log(response)
}

export async function setUsername(name: string) {
  const transactionId = await fcl.mutate({
    cadence: `
      import UserProfile from 0x6e6efd2c0e2ad3c3

      transaction(name: String) {
          prepare(signer: AuthAccount) {
              // Check if a \`User\` resource is already stored at the specified path
              if signer.borrow<&UserProfile.User>(from: /storage/userName) != nil {
                  // Remove the existing \`User\` resource and clean up storage
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
    `,
    args: (arg, t) => [arg(name, t.String)],
    proposer: await fcl.currentUser().authorization,
    payer: await fcl.currentUser().authorization,
    authorizations: [await fcl.currentUser().authorization],
  });

  return fcl.tx(transactionId).onceSealed();
}