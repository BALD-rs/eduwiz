import * as fcl from '@onflow/fcl'
import './config'
import { writable } from 'svelte/store'
import { browser } from '$app/environment'
import { goto } from '$app/navigation'

export async function logIn() {
  const u = await fcl.authenticate()

  if (u.loggedIn) {
    if (await getLevel() == 0) {
      goto('/onboard')
    } else {
      goto('/home')
    }
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

export async function getUsername(): Promise<string> {
  const address = (await fcl.currentUser.snapshot()).addr;
  let response;
  try {
    response = await fcl.query({
      cadence:`
      import UserProfile from 0x97c5b072d69e13b5

      pub fun main(user: Address): String {
        let auth: AuthAccount = getAuthAccount(user)
        let nameRef: &UserProfile.User = auth.borrow<&UserProfile.User>(from: /storage/userName) ?? panic("This User Does Not Exist!")
        return nameRef.name
      }    
        `,
        args: (arg, t) => [arg(address, t.Address)], // Adding the required argument
    })
  } catch (error) {
    response = "User"; // Return "User" if it panics
  }
  console.log(response)
  return response as string; // Ensuring the return type is string
}

export async function getLevel() {
  const address = (await fcl.currentUser.snapshot()).addr;
  let response;
  try {
    response = await fcl.query({
      cadence:`
      import UserProfile from 0x97c5b072d69e13b5

      pub fun main(user: Address): Int {
        let auth: AuthAccount = getAuthAccount(user)
        let nameRef: &UserProfile.User = auth.borrow<&UserProfile.User>(from: /storage/userName) ?? panic("This User Does Not Exist!")
        return nameRef.level
      }   
        `,
        args: (arg, t) => [arg(address, t.Address)], // Adding the required argument
    })
  } catch (error) {
    response = 0; // Return "User" if it panics
  }
  console.log(response)
  return response; // Ensuring the return type is string
}

export async function getQuizzesComplete() {
  const address = (await fcl.currentUser.snapshot()).addr;
  let response;
  try {
    response = await fcl.query({
      cadence:`
      import UserProfile from 0x97c5b072d69e13b5

      pub fun main(user: Address): Int {
        let auth: AuthAccount = getAuthAccount(user)
        let nameRef: &UserProfile.User = auth.borrow<&UserProfile.User>(from: /storage/userName) ?? panic("This User Does Not Exist!")
        return nameRef.quizzesCompleted
      }   
        `,
        args: (arg, t) => [arg(address, t.Address)], // Adding the required argument
    })
  } catch (error) {
    response = 0; // Return "User" if it panics
  }
  console.log(response)
  return response; // Ensuring the return type is string
}

export async function setUsername(name: string) {
  const transactionId = await fcl.mutate({
    cadence: `
    import UserProfile from 0x97c5b072d69e13b5

    transaction(name: String) {
      prepare(signer: AuthAccount) {
        // Try to borrow a reference to the User resource.
        // If it exists, update the name.
        if let userRef = signer.borrow<&UserProfile.User>(from: /storage/userName) {
          userRef.setName(name: name)
        } else {
          // If not, create a new User resource and store it.
          let newUser <- UserProfile.createUser(name: name)
          signer.save(<-newUser, to: /storage/userName)
        }
      }
      execute {
        log("User name set or updated.")
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

export async function quizComplete() {
  const transactionId = await fcl.mutate({
    cadence: `
    import UserProfile from 0x97c5b072d69e13b5

    transaction() {
      prepare(signer: AuthAccount) {
        // Try to borrow a reference to the User resource.
        // If it exists, update the name.
        if let userRef = signer.borrow<&UserProfile.User>(from: /storage/userName) {
          userRef.quizComplete()
        } else {
          log("no quiz complete")
        }
      }
      execute {
        log("quiz complete")
      }
    }
    `,
    proposer: await fcl.currentUser().authorization,
    payer: await fcl.currentUser().authorization,
    authorizations: [await fcl.currentUser().authorization],
  });

  return fcl.tx(transactionId).onceSealed();
}