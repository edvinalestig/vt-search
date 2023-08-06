# JourneyDetailsPeriodCallDetailsApiModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**stop_point** | [**crate::models::JourneyDetailsPeriodStopPointApiModel**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.StopPointApiModel.md) |  | 
**planned_arrival_time** | Option<**String**> | The planned arrival time for the call in RFC 3339 format. | [optional]
**planned_departure_time** | Option<**String**> | The planned departure time for the call in RFC 3339 format. | [optional]
**estimated_arrival_time** | Option<**String**> | The estimated arrival time for the call in RFC 3339 format. | [optional]
**estimated_departure_time** | Option<**String**> | The estimated departure time for the call in RFC 3339 format. | [optional]
**estimated_otherwise_planned_arrival_time** | Option<**String**> | The best known time of the call in RFC 3339 format. Is EstimatedArrivalTime if exists, otherwise PlannedArrivalTime. | [optional][readonly]
**estimated_otherwise_planned_departure_time** | Option<**String**> | The best known time of the call in RFC 3339 format. Is EstimatedDepartureTime if exists, otherwise PlannedDepartureTime. | [optional][readonly]
**planned_platform** | Option<**String**> | The planned platform of the call. | [optional]
**estimated_platform** | Option<**String**> | The estimated platform of the call. | [optional]
**latitude** | Option<**f64**> | The latitude of the stop point of the call. | [optional]
**longitude** | Option<**f64**> | The longitude of the stop point of the call. | [optional]
**index** | Option<**String**> | The index of the stop point of the call. | [optional]
**is_on_trip_leg** | Option<**bool**> | The call is on the trip leg. | [optional]
**is_trip_leg_start** | Option<**bool**> | The call is the first on the trip leg. | [optional]
**is_trip_leg_stop** | Option<**bool**> | The call is the last on the trip leg. | [optional]
**tariff_zones** | Option<[**Vec<crate::models::JourneyDetailsPeriodTariffZoneApiModel>**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.TariffZoneApiModel.md)> | The primary tariff zone of the call. A call can be related to a stop area with multiple tariff zones  and this is the zone that for example should be displayed in overviews etc. | [optional]
**occupancy** | Option<[**crate::models::OccupancyInformationApiModel**](VT.ApiPlaneraResa.Web.V4.Models.OccupancyInformationApiModel.md)> |  | [optional]
**is_cancelled** | Option<**bool**> | Flag indicating if the call is cancelled. | [optional]
**is_departure_cancelled** | Option<**bool**> | Flag indicating if the departure is cancelled. | [optional]
**is_arrival_cancelled** | Option<**bool**> | Flag indicating if the arrival is cancelled. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


