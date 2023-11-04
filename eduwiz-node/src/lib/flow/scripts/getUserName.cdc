import UserProfile from 0x01

pub fun main(user: Address): String {
  let auth: AuthAccount = getAuthAccount(user)
  let nameRef: &UserProfile.User = auth.borrow<&UserProfile.User>(from: /storage/userName) ?? panic("This User Does Not Exist!")
  return nameRef.name
}
