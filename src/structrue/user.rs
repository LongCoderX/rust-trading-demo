#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    status: AccountStatus
}

#[derive(Debug)]
enum AccountStatus {
    Active,
    Inactive,
    Banned(String),
}

impl std::fmt::Display for AccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountStatus::Active => {
                write!(f, "Active")
            },
            AccountStatus::Inactive => {
                write!(f, "Inactive")
            },
            AccountStatus::Banned(s) => {
                writeln!(f, "Banned")?;
                write!(f, "\tReason: {s}")
            }
        }
    }
}

impl User {
    fn new(name: String, age: u8) -> Self {
        Self {
            name : name,
            age : age,
            status: AccountStatus::Active
        }
    }

    fn ban(&mut self, reason: String) {
        self.status = AccountStatus::Banned(reason);
    }

    fn is_eligible(&self) -> bool {
        if self.age < 18 {
            return false;
        }

        match self.status {
            AccountStatus::Active => {
                return true;
            },
            _ => {
                return false;
            }
        }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\tName: {}", self.name)?;
        writeln!(f, "\tAge: {}", self.age)?;
        writeln!(f, "\tStatus: {}", self.status)
    }
}

pub fn test_user() {
    println!("============== No.3 Structure User and Enum AccountStatus ==============");
    
    let mut user = User::new(String::from("Tom"), 33);
    println!("The user is {user:?}");
    user.ban(String::from("G Dog."));
    println!("The user is {user:?}, and the eligible is {}.", user.is_eligible());
    println!("{user}");
    user.status = AccountStatus::Inactive;
    println!("The user is {user:?}, and the eligible is {}.", user.is_eligible());
    println!("{user}");
    user.status = AccountStatus::Active;
    println!("The user is {user:?}, and the eligible is {}.", user.is_eligible());
    println!("{user}");
}