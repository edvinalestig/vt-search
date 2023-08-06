# JourneysPeriodTripLegApiModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**origin** | [**crate::models::JourneysPeriodCallApiModel**](VT.ApiPlaneraResa.Web.V4.Models.Journeys.CallApiModel.md) |  | 
**destination** | [**crate::models::JourneysPeriodCallApiModel**](VT.ApiPlaneraResa.Web.V4.Models.Journeys.CallApiModel.md) |  | 
**is_cancelled** | **bool** | Flag indicating if the trip leg is cancelled. | 
**is_part_cancelled** | Option<**bool**> | Flag indicating if the trip leg is partially cancelled. | [optional]
**service_journey** | Option<[**crate::models::JourneysPeriodServiceJourneyApiModel**](VT.ApiPlaneraResa.Web.V4.Models.Journeys.ServiceJourneyApiModel.md)> |  | [optional]
**notes** | Option<[**Vec<crate::models::Note>**](VT.ApiPlaneraResa.Core.Models.Note.md)> | An ordered list (most important first) of notes related to the trip leg. | [optional]
**estimated_distance_in_meters** | Option<**i32**> | Estimated distance in meters. Only for transport mode Walk. | [optional]
**planned_connecting_time_in_minutes** | Option<**i32**> | The planned (according to timetable) connecting time in minutes relative to  the previous public transport trip leg (if any). | [optional]
**estimated_connecting_time_in_minutes** | Option<**i32**> | The estimated (according to real-time data) connecting time in minutes relative to  the previous public transport trip leg (if any). | [optional]
**is_risk_of_missing_connection** | Option<**bool**> | Flag indicating that there is less than 5 minutes margin between arriving at the  origin stop point and the departure from that stop point. | [optional]
**planned_departure_time** | Option<**String**> | The planned departure time in RFC 3339 format. | [optional]
**planned_arrival_time** | Option<**String**> | The planned arrival time in RFC 3339 format. | [optional]
**planned_duration_in_minutes** | Option<**i32**> | The planned duration in minutes. | [optional]
**estimated_departure_time** | Option<**String**> | The estimated departure time in RFC 3339 format, if available. | [optional]
**estimated_arrival_time** | Option<**String**> | The estimated arrival time in RFC 3339 format, if available. | [optional]
**estimated_duration_in_minutes** | Option<**i32**> | The estimated duration in minutes, if available. | [optional]
**estimated_otherwise_planned_arrival_time** | Option<**String**> | The best known time of the arrival in RFC 3339 format. Is EstimatedArrivalTime if exists, otherwise PlannedArrivalTime. | [optional][readonly]
**estimated_otherwise_planned_departure_time** | Option<**String**> | The best known time of the departure in RFC 3339 format. Is EstimatedDepartureTime if exists, otherwise PlannedDepartureTime. | [optional][readonly]
**occupancy** | Option<[**crate::models::OccupancyInformationApiModel**](VT.ApiPlaneraResa.Web.V4.Models.OccupancyInformationApiModel.md)> |  | [optional]
**journey_leg_index** | Option<**i32**> | Index of Leg in Journey | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


