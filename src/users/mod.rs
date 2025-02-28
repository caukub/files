use uuid::Uuid;

struct User {
    uuid: Uuid,
}

enum UserError {
    NotFound,
}

enum Permission {}

trait UserDataDriver {
    fn create_user() -> Result<(), UserError>;
    fn delete_user(uuid: Uuid) -> Result<(), UserError>;
    fn get_user(uuid: Uuid) -> User;
    fn update_user(user: User) -> Result<(), UserError>;
    fn has_permission(permission: Permission, _server: ()) -> Result<bool, UserError>;
}
