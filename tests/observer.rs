#![cfg(feature = "observer")]
use mdd::observer;

observer! {
    pub enum Event {
        Created { id: uuid::Uuid, name: String },
        Updated { id: uuid::Uuid, new_name: String },
        Deleted { id: uuid::Uuid },
    }
}

#[derive(Clone)]
pub struct Obs {
    uuid: uuid::Uuid,
}

#[async_trait]
impl Believer for Obs {
    async fn destiny(&self, event: &mut Event) -> Unknow {
        match event {
            Event::Created { id, name } => Box::new(0),
            Event::Deleted { id } => Box::new(1),
            Event::Updated { id, new_name } => Box::new(2),
        }
    }
}

#[tokio::test]
async fn event_deleted() {
    let mut god = God::new();
    let obs = Obs {
        uuid: uuid::Uuid::new_v4(),
    };
    god.watch(obs.uuid.clone().to_string(), Box::new(obs.clone()));
    let mut destiny = god.listen(&mut Event::Deleted { id: obs.uuid }).await;
    destiny.set_key(obs.uuid.to_string().into());
    let result = destiny.get_believer_destiny_way_by_key::<i32>();
    assert_eq!(result, Some(1));
}

#[tokio::test]
async fn event_created() {
    let mut god = God::new();
    let obs = Obs {
        uuid: uuid::Uuid::new_v4(),
    };
    god.watch(obs.uuid.clone().to_string(), Box::new(obs.clone()));
    let mut destiny = god
        .listen(&mut Event::Created {
            id: obs.uuid,
            name: "User".to_string(),
        })
        .await;
    destiny.set_key(obs.uuid.to_string().into());
    let result = destiny.get_believer_destiny_way_by_key::<i32>();
    assert_eq!(result, Some(0));
}

#[tokio::test]
async fn event_updated() {
    let mut god = God::new();
    let obs = Obs {
        uuid: uuid::Uuid::new_v4(),
    };
    god.watch(obs.uuid.clone().to_string(), Box::new(obs.clone()));
    let mut destiny = god
        .listen(&mut Event::Updated {
            id: obs.uuid,
            new_name: "User".to_string(),
        })
        .await;
    destiny.set_key(obs.uuid.to_string().into());
    let result = destiny.get_believer_destiny_way_by_key::<i32>();
    assert_eq!(result, Some(2));
}
