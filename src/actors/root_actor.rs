/// This is a template for root actors. 
/// We qualify a `root actor` as teh root of an actor hierarchy, with its own threads.
///
/// Note that there are only two root actors:
///
///   * The `user actor` which is the root for all user created actors.
///   * The `system actor` which is the root for all actors created for the actor system.

use std::any::Any;
use std::sync::Arc;

use actors::{Actor, ActorCell, ActorContext, ActorPath, ActorRef, ActorSystem, Arguments,
             CanReceive, InnerMessage, Props, SystemMessage};
use actors::cthulhu::Cthulhu;

pub struct RootActorRef {
    actor_cell: ActorCell<(), InternalRootActor>,
    path: ActorPath,
}

impl RootActorRef {
    /// Creates a RootActor.
    pub fn new(system: ActorSystem, name: String, cthulhu: Arc<Cthulhu>) -> RootActorRef {
        let props = Props::new(Arc::new(InternalRootActor::new), ());
        let actor = props.create();
        let name = Arc::new(name);
        let actor_cell = ActorCell::new(actor, props, system, cthulhu, name.clone(), name.clone());
        RootActorRef {
            actor_cell: actor_cell,
            path: name.clone(),
        }
    }

    /// Creates an actor for the root.
    pub fn actor_of<Args: Arguments, A: Actor + 'static>(&self,
                                                         props: Props<Args, A>,
                                                         name: String)
                                                         -> Arc<ActorRef<Args, A>> {
        self.actor_cell.actor_of(props, name)
    }
}

impl Clone for RootActorRef {
    fn clone(&self) -> RootActorRef {
        RootActorRef {
            actor_cell: self.actor_cell.clone(),
            path: self.path.clone(),
        }
    }
}

// FIXME(gamazeps) this is a copy of the code in src/actor_ref.rs, this is bad.
impl CanReceive for RootActorRef {
    fn receive(&self, message: InnerMessage, sender: Arc<CanReceive>) {
        self.actor_cell.receive_message(message, sender);
    }

    fn receive_system_message(&self, system_message: SystemMessage) {
        self.actor_cell.receive_system_message(system_message);
    }

    fn handle(&self) {
        self.actor_cell.handle_envelope();
    }

    fn path(&self) -> ActorPath {
        self.path.clone()
    }
}

struct InternalRootActor;

impl InternalRootActor {
    fn new(_dummy: ()) -> InternalRootActor {
        InternalRootActor
    }
}

impl Actor for InternalRootActor {
    // The receive function is currently a dummy.
    fn receive<Args: Arguments>(&self,
                                _message: Box<Any>,
                                _context: ActorCell<Args, InternalRootActor>) {
    }
}