use std::fs;
use text_io::scan;

#[derive(Clone)]
struct ResourcesState(usize, usize, usize, usize);
#[derive(Clone, Eq, Hash, PartialEq)]
struct BuildingsState(usize, usize, usize, usize);
#[derive(Clone)]
struct StructureCost(usize, usize, usize);
#[derive(Clone)]
struct Blueprint(StructureCost, StructureCost, StructureCost, StructureCost);

fn parse_input(input: &String) -> Vec<Blueprint> {
    input.lines().map(|line| {
        let blueprint_i: usize;
        let mut ore_robot_cost = StructureCost(0,0,0);
        let mut clay_robot_cost = StructureCost(0,0,0);
        let mut obsidian_robot_cost = StructureCost(0,0,0);
        let mut geode_robot_cost = StructureCost(0,0,0);

        scan!(line.bytes() => "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.", blueprint_i, ore_robot_cost.0, clay_robot_cost.0, obsidian_robot_cost.0, obsidian_robot_cost.1, geode_robot_cost.0, geode_robot_cost.2);

        Blueprint(ore_robot_cost, clay_robot_cost, obsidian_robot_cost, geode_robot_cost)
    }).collect::<Vec<_>>()
}

fn geodes_per_minute(blueprint: &Blueprint, cycles_count: usize) -> usize {
    let mut found_max: usize = 0;

    let mut dfs_stack: Vec<(usize, ResourcesState, BuildingsState, BuildingsState)> = vec![(
        cycles_count,
        ResourcesState(0, 0, 0, 0),
        BuildingsState(1, 0, 0, 0),
        BuildingsState(0, 0, 0, 0),
    )];

    while !dfs_stack.is_empty() {
        let (cycles_remaining, resources, buildings, construction_que) = dfs_stack.pop().unwrap();

        let new_resources = ResourcesState(
            resources.0 + buildings.0,
            resources.1 + buildings.1,
            resources.2 + buildings.2,
            resources.3 + buildings.3,
        );

        if cycles_remaining == 1 {
            found_max = if new_resources.3 > found_max {
                new_resources.3
            } else {
                found_max
            };
            continue;
        }

        let new_buildings = BuildingsState(
            buildings.0 + construction_que.0,
            buildings.1 + construction_que.1,
            buildings.2 + construction_que.2,
            buildings.3 + construction_que.3,
        );

        // Remove branches that have no optimistic chance in producing better result
        // (assume you will create geode factory every step)
        let potential_optimistic_production = if can_afford(&blueprint.3, &new_resources) {
            cycles_remaining * (cycles_remaining - 1) / 2
        } else {
            (cycles_remaining - 1) * (cycles_remaining - 2) / 2
        };
        let potential_geodes = (cycles_remaining - 1) * new_buildings.3
            + potential_optimistic_production
            + new_resources.3;

        if potential_geodes < found_max {
            continue;
        }

        if can_afford(&blueprint.3, &new_resources) {
            let mut next_resources = new_resources.clone();
            next_resources.0 -= blueprint.3 .0;
            next_resources.1 -= blueprint.3 .1;
            next_resources.2 -= blueprint.3 .2;
            dfs_stack.push((
                cycles_remaining - 1,
                next_resources,
                new_buildings.clone(),
                BuildingsState(0, 0, 0, 1),
            ));
        } else {
            if can_afford(&blueprint.2, &new_resources)
                && !produces_enough_obsiadian(&new_buildings, blueprint)
            {
                let mut next_resources = new_resources.clone();
                next_resources.0 -= blueprint.2 .0;
                next_resources.1 -= blueprint.2 .1;
                next_resources.2 -= blueprint.2 .2;
                dfs_stack.push((
                    cycles_remaining - 1,
                    next_resources,
                    new_buildings.clone(),
                    BuildingsState(0, 0, 1, 0),
                ));
            }

            if can_afford(&blueprint.1, &new_resources)
                && !produces_enough_clay(&new_buildings, blueprint)
            {
                let mut next_resources = new_resources.clone();
                next_resources.0 -= blueprint.1 .0;
                next_resources.1 -= blueprint.1 .1;
                next_resources.2 -= blueprint.1 .2;
                dfs_stack.push((
                    cycles_remaining - 1,
                    next_resources,
                    new_buildings.clone(),
                    BuildingsState(0, 1, 0, 0),
                ));
            }
            if can_afford(&blueprint.0, &new_resources)
                && !produces_enough_ore(&new_buildings, blueprint)
            {
                let mut next_resources = new_resources.clone();
                next_resources.0 -= blueprint.0 .0;
                next_resources.1 -= blueprint.0 .1;
                next_resources.2 -= blueprint.0 .2;
                dfs_stack.push((
                    cycles_remaining - 1,
                    next_resources,
                    new_buildings.clone(),
                    BuildingsState(1, 0, 0, 0),
                ));
            }
            dfs_stack.push((
                cycles_remaining - 1,
                new_resources,
                new_buildings.clone(),
                BuildingsState(0, 0, 0, 0),
            ));
        }
    }
    found_max
}

fn can_afford(structure: &StructureCost, resources: &ResourcesState) -> bool {
    resources.0 >= structure.0 && resources.1 >= structure.1 && resources.2 >= structure.2
}

fn produces_enough_obsiadian(buildings: &BuildingsState, blueprint: &Blueprint) -> bool {
    buildings.2
        >= [
            blueprint.0 .2,
            blueprint.1 .2,
            blueprint.2 .2,
            blueprint.3 .2,
        ]
        .iter()
        .max()
        .unwrap()
        .clone()
}

fn produces_enough_clay(buildings: &BuildingsState, blueprint: &Blueprint) -> bool {
    buildings.1
        >= [
            blueprint.0 .1,
            blueprint.1 .1,
            blueprint.2 .1,
            blueprint.3 .1,
        ]
        .iter()
        .max()
        .unwrap()
        .clone()
}

fn produces_enough_ore(buildings: &BuildingsState, blueprint: &Blueprint) -> bool {
    buildings.0
        >= [
            blueprint.0 .0,
            blueprint.1 .0,
            blueprint.2 .0,
            blueprint.3 .0,
        ]
        .iter()
        .max()
        .unwrap()
        .clone()
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");
    let blueprints = parse_input(&input);

    let result_1 = blueprints
        .iter()
        .map(|b| geodes_per_minute(&b, 24))
        .enumerate()
        .map(|(i, val)| (i + 1) * val.clone())
        .sum::<usize>();
    println!("{:?}", result_1);

    let result_2 = blueprints
        .iter()
        .take(3)
        .map(|b| geodes_per_minute(&b, 32))
        .product::<usize>();
    println!("{:?}", result_2);
}
