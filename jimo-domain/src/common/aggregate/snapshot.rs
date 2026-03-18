use crate::common::{Aggregate, aggregate::error::SnapShotError};

pub struct SnapShot<A>
where
    A: Aggregate,
{
    aggregate: A,
    version: u64,
}

impl<A> SnapShot<A>
where
    A: Aggregate,
{
    pub fn id(&self) -> &A::ID {
        self.aggregate.id()
    }

    pub fn to_aggregate(self) -> A {
        self.aggregate
    }

    pub fn version(&self) -> u64 {
        self.version
    }
}

pub trait SnapShotStore<A>
where
    A: Aggregate,
{
    async fn save(&self, snapshot: SnapShot<A>) -> Result<(), SnapShotError>;

    async fn get(&self, id: &A::ID) -> Result<Option<SnapShot<A>>, SnapShotError>;
}
