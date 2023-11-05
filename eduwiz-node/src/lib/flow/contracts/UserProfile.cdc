pub contract UserProfile {

    pub resource User {
        pub var name: String
        pub var level: Int
        pub var quizzesCompleted: Int

        init(name: String) {
            self.name = name
            self.level = 1
            self.quizzesCompleted = 0
        }

        destroy() {
            
        }

        pub fun getName(): String {
            return self.name
        }

        pub fun setName(name: String) {
            self.name = name
        }

        pub fun getLevel(): Int {
            return self.level
        }

        pub fun quizComplete() {
            self.quizzesCompleted = self.quizzesCompleted + 1
            
            if self.quizzesCompleted >= 5 {
                self.level = 5
            } else if self.quizzesCompleted >= 4 {
                self.level = 4
            } else if self.quizzesCompleted >= 3 {
                self.level = 3
            } else if self.quizzesCompleted >= 2 {
                self.level = 2
            } else {
                self.level = 1
            }
        }

        pub fun getNumQuiz(): Int {
            return self.quizzesCompleted
        }
    }

    pub fun createUser(name: String): @User {
        return <- create User(name: name)
    }

    init() {
        log("User Created")
    }
}