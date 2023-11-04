pub contract UserProfile {

    pub resource User {
        pub var name: String

        init(name: String) {
            self.name = name
        }

        destroy() {
            
        }

        pub fun getName(): String {
            return self.name
        }
    }

    pub fun createUser(name: String): @User {
        return <- create User(name: name)
    }

    init() {
        log("User Created")
    }
}