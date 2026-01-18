#[cfg(test)]
mod tests {
    use abi::Graphable;
    use abi::types::*;

    #[test]
    fn inventory_check() {
        // Assertions for canonical types
        // Edge
        assert_eq!(core::mem::size_of::<Edge>(), 52); // 16*3 + 4 = 52
        assert_eq!(Edge::SCHEMA.size(), 52);
        
        // Window
        assert_eq!(core::mem::size_of::<Window>(), 32); // 16 + 4 + 4 + 4 + 4 = 32
        assert_eq!(Window::SCHEMA.size(), 32);

        // Kind
        assert_eq!(core::mem::size_of::<Kind>(), 32); // 16 + 16 = 32
        assert_eq!(Kind::SCHEMA.size(), 32);

        // Predicate
        assert_eq!(core::mem::size_of::<Predicate>(), 32);
        assert_eq!(Predicate::SCHEMA.size(), 32);

        // LogEvent
        assert_eq!(core::mem::size_of::<LogEvent>(), 44); // 16 + 8 + 4 + 16 = 44
        assert_eq!(LogEvent::SCHEMA.size(), 44);

        // Process
        assert_eq!(core::mem::size_of::<Process>(), 40); // 16 + 16 + 8
        assert_eq!(Process::SCHEMA.size(), 40);

        // Thread
        assert_eq!(core::mem::size_of::<Thread>(), 40); // 16 + 16 + 8
        assert_eq!(Thread::SCHEMA.size(), 40);

        // Task
        assert_eq!(core::mem::size_of::<Task>(), 52); // 16 + 16 + 16 + 4
        assert_eq!(Task::SCHEMA.size(), 52);

        // Asset
        assert_eq!(core::mem::size_of::<Asset>(), 64); // 16 + 16 + 16 + 16
        assert_eq!(Asset::SCHEMA.size(), 64);

        // Font
        assert_eq!(core::mem::size_of::<Font>(), 64); // 16 + 16 + 16 + 16
        assert_eq!(Font::SCHEMA.size(), 64);
    }
}
