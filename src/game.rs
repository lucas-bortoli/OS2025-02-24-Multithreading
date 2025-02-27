pub enum Room {
    Room427,
    Hallway,
    MeetingRoom,
    EmployeeLounge,
}

pub enum RoomAction {
    GoToRoom427,
    GoToHallway,
    GoToMeetingRoom,
    GoToEmployeeLounge,
}

pub struct RoomDescription {
    id: Room,
    name: String,
    description: String,
    actions: Vec<(RoomAction, String)>,
}

pub fn room_lookup(room: &Room) -> RoomDescription {
    return match room {
        Room::Room427 => RoomDescription {
            id: Room::Room427,
            name: "Sala 427".to_string(),
            description: "Você está na sua mesa de trabalho, em um grande prédio corporativo. O som constante do sistema de ventilação faz com que a sala pareça calma, quase entorpecida. Sua mesa contém uma simples tela de computador e uma pilha de documentos, sem janelas à vista, o ambiente parece estático. Atrás de você, a porta está ligeiramente entreaberta.".to_string(),
            actions: vec![(RoomAction::GoToHallway, "Sair do escritório".to_string())],
        },
        Room::Hallway => RoomDescription {
            id: Room::Hallway,
            name: "Hall de entrada".to_string(),
            description: "Você está no longo hall de entrada que junta o seu escritório aos dos colegas. O chão brilhante parece polido há muito tempo e há gaveteiros espalhados pelo chão. Na sua frente você pode ver duas portas; à direita, uma porta indica o acesso à sala de reunião, e à esquerda, uma porta para a sala de recreação.".to_string(),
            actions: vec![
                (RoomAction::GoToMeetingRoom, "Entrar na porta da esquerda".to_string()),
                (RoomAction::GoToEmployeeLounge, "Entrar na porta da direita".to_string()),
                (RoomAction::GoToRoom427, "Voltar ao meu escritório".to_string())
            ],
        },
        Room::MeetingRoom => RoomDescription {
            id: Room::MeetingRoom,
            name: "Sala de reuniões".to_string(),
            description: "Uma longa mesa de reuniões fica no centro da sala. De cada lado há um pequeno sofá com poltrona de couro macio, e na parede estão pendurados vários cartazes corporativos, além de uma porta para uma área de serviço. No projetor, há uma apresentação de PowerPoint em pausa com uma imagem de gráficos de receita e lucros. Na tela: \"Empregue a solução A.\" Um cheiro levemente enjoativo está pairando na sala. Você se lembra dos cheiros do ambiente: as notas amargas dos desinfetantes corporativos.".to_string(),
            actions: vec![
                (RoomAction::GoToHallway, "Voltar ao hall".to_string())
            ],
        },
        Room::EmployeeLounge => RoomDescription {
            id: Room::EmployeeLounge,
            name: "Sala de recreação".to_string(),
            description: "Um espaço para relaxamento, há um sofá velho e desgastado no meio da sala, uma chaleira com café quente sobre uma pequena mesinha. O som de uma máquina de venda automática é constante.".to_string(),
            actions: vec![
                (RoomAction::GoToHallway, "Voltar ao hall".to_string())
            ],
        },
    };
}

pub struct State {
    pub current_room: Room,
}

impl State {
    pub fn new() -> State {
        State {
            current_room: Room::Room427,
        }
    }

    pub fn describe(&self) {
        let description = room_lookup(&self.current_room);

        println!();
        println!("{}", description.description);
        println!();
        println!("O que você deseja fazer?");
        for (index, (_, action_label)) in description.actions.iter().enumerate() {
            println!("{: >5}  {}", index + 1, action_label);
        }
        println!("\"sair\" para fechar o programa");
    }

    pub fn trigger_room_action(&mut self, action_screen_number: usize) -> Result<(), &str> {
        let description = room_lookup(&self.current_room);
        if action_screen_number == 0 || action_screen_number > description.actions.len() {
            return Err("Escolha inválida.");
        }

        let (action, _) = &description.actions.get(action_screen_number - 1).unwrap();

        match action {
            RoomAction::GoToRoom427 => self.current_room = Room::Room427,
            RoomAction::GoToHallway => self.current_room = Room::Hallway,
            RoomAction::GoToMeetingRoom => self.current_room = Room::MeetingRoom,
            RoomAction::GoToEmployeeLounge => self.current_room = Room::EmployeeLounge,
        }

        return Ok(());
    }
}
