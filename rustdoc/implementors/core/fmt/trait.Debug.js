(function() {var implementors = {};
implementors["abstutil"] = [{"text":"impl Debug for MapName","synthetic":false,"types":[]},{"text":"impl Debug for Tags","synthetic":false,"types":[]},{"text":"impl Debug for Progress","synthetic":false,"types":[]}];
implementors["collisions"] = [{"text":"impl Debug for Severity","synthetic":false,"types":[]}];
implementors["game"] = [{"text":"impl Debug for SEIR","synthetic":false,"types":[]},{"text":"impl Debug for DashTab","synthetic":false,"types":[]}];
implementors["geom"] = [{"text":"impl Debug for Angle","synthetic":false,"types":[]},{"text":"impl Debug for Bounds","synthetic":false,"types":[]},{"text":"impl Debug for GPSBounds","synthetic":false,"types":[]},{"text":"impl Debug for Circle","synthetic":false,"types":[]},{"text":"impl Debug for Distance","synthetic":false,"types":[]},{"text":"impl Debug for Duration","synthetic":false,"types":[]},{"text":"impl Debug for LonLat","synthetic":false,"types":[]},{"text":"impl Debug for Line","synthetic":false,"types":[]},{"text":"impl Debug for InfiniteLine","synthetic":false,"types":[]},{"text":"impl Debug for Polygon","synthetic":false,"types":[]},{"text":"impl Debug for Triangle","synthetic":false,"types":[]},{"text":"impl Debug for PolyLine","synthetic":false,"types":[]},{"text":"impl Debug for Pt2D","synthetic":false,"types":[]},{"text":"impl Debug for HashablePt2D","synthetic":false,"types":[]},{"text":"impl Debug for Ring","synthetic":false,"types":[]},{"text":"impl Debug for Speed","synthetic":false,"types":[]},{"text":"impl Debug for Time","synthetic":false,"types":[]}];
implementors["importer"] = [{"text":"impl Debug for Record","synthetic":false,"types":[]},{"text":"impl Debug for TripRecord","synthetic":false,"types":[]},{"text":"impl Debug for StopTimeRecord","synthetic":false,"types":[]}];
implementors["kml"] = [{"text":"impl Debug for ExtraShape","synthetic":false,"types":[]}];
implementors["map_editor"] = [{"text":"impl Debug for ID","synthetic":false,"types":[]}];
implementors["map_gui"] = [{"text":"impl Debug for ColorSchemeChoice","synthetic":false,"types":[]},{"text":"impl Debug for TrafficSignalStyle","synthetic":false,"types":[]},{"text":"impl Debug for CameraAngle","synthetic":false,"types":[]},{"text":"impl Debug for ID","synthetic":false,"types":[]}];
implementors["map_model"] = [{"text":"impl Debug for OriginalLane","synthetic":false,"types":[]},{"text":"impl Debug for ChangeLaneType","synthetic":false,"types":[]},{"text":"impl Debug for ReverseLane","synthetic":false,"types":[]},{"text":"impl Debug for ChangeSpeedLimit","synthetic":false,"types":[]},{"text":"impl Debug for ChangeAccessRestrictions","synthetic":false,"types":[]},{"text":"impl Debug for MapEdits","synthetic":false,"types":[]},{"text":"impl Debug for EditIntersection","synthetic":false,"types":[]},{"text":"impl Debug for EditRoad","synthetic":false,"types":[]},{"text":"impl Debug for EditCmd","synthetic":false,"types":[]},{"text":"impl Debug for MapConfig","synthetic":false,"types":[]},{"text":"impl Debug for DrivingSide","synthetic":false,"types":[]},{"text":"impl Debug for AreaID","synthetic":false,"types":[]},{"text":"impl Debug for AreaType","synthetic":false,"types":[]},{"text":"impl Debug for Area","synthetic":false,"types":[]},{"text":"impl Debug for BuildingID","synthetic":false,"types":[]},{"text":"impl Debug for Building","synthetic":false,"types":[]},{"text":"impl Debug for Amenity","synthetic":false,"types":[]},{"text":"impl Debug for OffstreetParking","synthetic":false,"types":[]},{"text":"impl Debug for BuildingType","synthetic":false,"types":[]},{"text":"impl Debug for NamePerLanguage","synthetic":false,"types":[]},{"text":"impl Debug for BusStopID","synthetic":false,"types":[]},{"text":"impl Debug for BusRouteID","synthetic":false,"types":[]},{"text":"impl Debug for BusStop","synthetic":false,"types":[]},{"text":"impl Debug for BusRoute","synthetic":false,"types":[]},{"text":"impl Debug for IntersectionID","synthetic":false,"types":[]},{"text":"impl Debug for IntersectionType","synthetic":false,"types":[]},{"text":"impl Debug for Intersection","synthetic":false,"types":[]},{"text":"impl Debug for LaneID","synthetic":false,"types":[]},{"text":"impl Debug for LaneType","synthetic":false,"types":[]},{"text":"impl Debug for Lane","synthetic":false,"types":[]},{"text":"impl Debug for ParkingLotID","synthetic":false,"types":[]},{"text":"impl Debug for RoadID","synthetic":false,"types":[]},{"text":"impl Debug for Direction","synthetic":false,"types":[]},{"text":"impl Debug for DirectedRoadID","synthetic":false,"types":[]},{"text":"impl Debug for Road","synthetic":false,"types":[]},{"text":"impl Debug for ControlStopSign","synthetic":false,"types":[]},{"text":"impl Debug for RoadWithStopSign","synthetic":false,"types":[]},{"text":"impl Debug for ControlTrafficSignal","synthetic":false,"types":[]},{"text":"impl Debug for Stage","synthetic":false,"types":[]},{"text":"impl Debug for PhaseType","synthetic":false,"types":[]},{"text":"impl Debug for TurnID","synthetic":false,"types":[]},{"text":"impl Debug for TurnType","synthetic":false,"types":[]},{"text":"impl Debug for TurnPriority","synthetic":false,"types":[]},{"text":"impl Debug for Turn","synthetic":false,"types":[]},{"text":"impl Debug for MovementID","synthetic":false,"types":[]},{"text":"impl Debug for CompressedMovementID","synthetic":false,"types":[]},{"text":"impl Debug for Movement","synthetic":false,"types":[]},{"text":"impl Debug for AccessRestrictions","synthetic":false,"types":[]},{"text":"impl Debug for Zone","synthetic":false,"types":[]},{"text":"impl Debug for NodeID","synthetic":false,"types":[]},{"text":"impl Debug for WayID","synthetic":false,"types":[]},{"text":"impl Debug for RelationID","synthetic":false,"types":[]},{"text":"impl Debug for OsmID","synthetic":false,"types":[]},{"text":"impl Debug for Node","synthetic":false,"types":[]},{"text":"impl Debug for UberTurn","synthetic":false,"types":[]},{"text":"impl Debug for WalkingNode","synthetic":false,"types":[]},{"text":"impl Debug for PathStep","synthetic":false,"types":[]},{"text":"impl Debug for Path","synthetic":false,"types":[]},{"text":"impl Debug for PathConstraints","synthetic":false,"types":[]},{"text":"impl Debug for PathRequest","synthetic":false,"types":[]},{"text":"impl Debug for RawMap","synthetic":false,"types":[]},{"text":"impl Debug for OriginalRoad","synthetic":false,"types":[]},{"text":"impl Debug for RawRoad","synthetic":false,"types":[]},{"text":"impl Debug for RawIntersection","synthetic":false,"types":[]},{"text":"impl Debug for RawBuilding","synthetic":false,"types":[]},{"text":"impl Debug for RawArea","synthetic":false,"types":[]},{"text":"impl Debug for RawParkingLot","synthetic":false,"types":[]},{"text":"impl Debug for RestrictionType","synthetic":false,"types":[]},{"text":"impl Debug for TurnRestriction","synthetic":false,"types":[]},{"text":"impl Debug for RawBusRoute","synthetic":false,"types":[]},{"text":"impl Debug for RawBusStop","synthetic":false,"types":[]},{"text":"impl Debug for Position","synthetic":false,"types":[]},{"text":"impl Debug for Traversable","synthetic":false,"types":[]}];
implementors["parking_mapper"] = [{"text":"impl Debug for Show","synthetic":false,"types":[]}];
implementors["sim"] = [{"text":"impl Debug for TripPhase","synthetic":false,"types":[]},{"text":"impl Debug for Event","synthetic":false,"types":[]},{"text":"impl Debug for AlertLocation","synthetic":false,"types":[]},{"text":"impl Debug for TripPhaseType","synthetic":false,"types":[]},{"text":"impl Debug for ScenarioGenerator","synthetic":false,"types":[]},{"text":"impl Debug for SpawnOverTime","synthetic":false,"types":[]},{"text":"impl Debug for BorderSpawnOverTime","synthetic":false,"types":[]},{"text":"impl Debug for Scenario","synthetic":false,"types":[]},{"text":"impl Debug for PersonSpec","synthetic":false,"types":[]},{"text":"impl Debug for IndividTrip","synthetic":false,"types":[]},{"text":"impl Debug for TripPurpose","synthetic":false,"types":[]},{"text":"impl Debug for TripSpec","synthetic":false,"types":[]},{"text":"impl Debug for Car","synthetic":false,"types":[]},{"text":"impl Debug for CarState","synthetic":false,"types":[]},{"text":"impl Debug for Request","synthetic":false,"types":[]},{"text":"impl Debug for PedState","synthetic":false,"types":[]},{"text":"impl Debug for Cmd","synthetic":false,"types":[]},{"text":"impl Debug for AnyTime","synthetic":false,"types":[]},{"text":"impl Debug for StateEvent","synthetic":false,"types":[]},{"text":"impl Debug for Event","synthetic":false,"types":[]},{"text":"impl Debug for State","synthetic":false,"types":[]},{"text":"impl Debug for Router","synthetic":false,"types":[]},{"text":"impl Debug for ActionAtEnd","synthetic":false,"types":[]},{"text":"impl Debug for Goal","synthetic":false,"types":[]},{"text":"impl Debug for Command","synthetic":false,"types":[]},{"text":"impl Debug for CommandType","synthetic":false,"types":[]},{"text":"impl Debug for SimpleCommandType","synthetic":false,"types":[]},{"text":"impl Debug for TripManager","synthetic":false,"types":[]},{"text":"impl Debug for Trip","synthetic":false,"types":[]},{"text":"impl Debug for TripInfo","synthetic":false,"types":[]},{"text":"impl Debug for TripLeg","synthetic":false,"types":[]},{"text":"impl Debug for TripMode","synthetic":false,"types":[]},{"text":"impl Debug for TripEndpoint","synthetic":false,"types":[]},{"text":"impl Debug for Person","synthetic":false,"types":[]},{"text":"impl Debug for PersonState","synthetic":false,"types":[]},{"text":"impl Debug for CarID","synthetic":false,"types":[]},{"text":"impl Debug for PedestrianID","synthetic":false,"types":[]},{"text":"impl Debug for AgentID","synthetic":false,"types":[]},{"text":"impl Debug for AgentType","synthetic":false,"types":[]},{"text":"impl Debug for TripID","synthetic":false,"types":[]},{"text":"impl Debug for PersonID","synthetic":false,"types":[]},{"text":"impl Debug for OrigPersonID","synthetic":false,"types":[]},{"text":"impl Debug for VehicleType","synthetic":false,"types":[]},{"text":"impl Debug for Vehicle","synthetic":false,"types":[]},{"text":"impl Debug for VehicleSpec","synthetic":false,"types":[]},{"text":"impl Debug for ParkingSpot","synthetic":false,"types":[]},{"text":"impl Debug for ParkedCar","synthetic":false,"types":[]},{"text":"impl Debug for DrivingGoal","synthetic":false,"types":[]},{"text":"impl Debug for SidewalkSpot","synthetic":false,"types":[]},{"text":"impl Debug for SidewalkPOI","synthetic":false,"types":[]},{"text":"impl Debug for TimeInterval","synthetic":false,"types":[]},{"text":"impl Debug for DistanceInterval","synthetic":false,"types":[]},{"text":"impl Debug for CreatePedestrian","synthetic":false,"types":[]},{"text":"impl Debug for CreateCar","synthetic":false,"types":[]}];
implementors["widgetry"] = [{"text":"impl Debug for HorizontalAlignment","synthetic":false,"types":[]},{"text":"impl Debug for VerticalAlignment","synthetic":false,"types":[]},{"text":"impl Debug for CameraState","synthetic":false,"types":[]},{"text":"impl Debug for Color","synthetic":false,"types":[]},{"text":"impl Debug for Fill","synthetic":false,"types":[]},{"text":"impl Debug for Texture","synthetic":false,"types":[]},{"text":"impl Debug for LinearGradient","synthetic":false,"types":[]},{"text":"impl Debug for Uniforms","synthetic":false,"types":[]},{"text":"impl Debug for Event","synthetic":false,"types":[]},{"text":"impl Debug for Key","synthetic":false,"types":[]},{"text":"impl Debug for MultiKey","synthetic":false,"types":[]},{"text":"impl Debug for UpdateType","synthetic":false,"types":[]},{"text":"impl Debug for ScreenPt","synthetic":false,"types":[]},{"text":"impl Debug for ScreenRectangle","synthetic":false,"types":[]},{"text":"impl Debug for ScreenDims","synthetic":false,"types":[]},{"text":"impl Debug for Font","synthetic":false,"types":[]},{"text":"impl Debug for TextSpan","synthetic":false,"types":[]},{"text":"impl Debug for Text","synthetic":false,"types":[]},{"text":"impl Debug for Outcome","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()