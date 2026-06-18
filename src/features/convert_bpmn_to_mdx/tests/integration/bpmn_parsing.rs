use detent::features::convert_bpmn_to_mdx::adapters::bpmn::parse_bpmn;

#[test]
fn parse_preserves_user_and_manual_tasks() {
    let xml = r#"<bpmn:definitions id="d" xmlns:bpmn="http://www.omg.org/spec/BPMN/20100524/MODEL">
    <bpmn:process id="p">
        <bpmn:userTask id="ut1"><bpmn:incoming>f1</bpmn:incoming></bpmn:userTask>
        <bpmn:manualTask id="mt1"><bpmn:incoming>f2</bpmn:incoming></bpmn:manualTask>
        <bpmn:serviceTask id="st1"/>
    </bpmn:process>
</bpmn:definitions>"#;

    let defs = parse_bpmn(xml).expect("parse should succeed");
    let proc = defs.process.expect("should have process");
    assert_eq!(proc.user_tasks.len(), 1, "should have one userTask");
    assert_eq!(proc.user_tasks[0].id, "ut1");
    assert_eq!(proc.manual_tasks.len(), 1, "should have one manualTask");
    assert_eq!(proc.manual_tasks[0].id, "mt1");
    assert_eq!(proc.service_tasks.len(), 1, "should have one serviceTask");
    assert_eq!(proc.service_tasks[0].id, "st1");
}

#[test]
fn parse_strips_unsupported_layout_elements() {
    let xml = r#"<bpmn:definitions id="d" xmlns:bpmn="http://www.omg.org/spec/BPMN/20100524/MODEL">
    <bpmn:process id="p">
        <bpmn:laneSet id="lanes"><bpmn:lane id="lane1"/></bpmn:laneSet>
        <bpmn:serviceTask id="st1"/>
        <bpmn:serviceTask id="st2"/>
    </bpmn:process>
</bpmn:definitions>"#;

    let defs = parse_bpmn(xml).expect("parse should succeed after stripping laneSet");
    let proc = defs.process.expect("should have process");
    assert_eq!(proc.service_tasks.len(), 2, "both serviceTasks should remain");
    assert_eq!(proc.service_tasks[0].id, "st1");
    assert_eq!(proc.service_tasks[1].id, "st2");
}
