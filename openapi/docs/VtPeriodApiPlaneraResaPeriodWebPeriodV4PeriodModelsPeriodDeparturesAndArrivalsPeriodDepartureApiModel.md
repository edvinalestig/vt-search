# DeparturesAndArrivalsPeriodDepartureApiModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**details_reference** | Option<**String**> | A reference that should be used when getting detailed information about the journey. | [optional]
**service_journey** | Option<[**crate::models::DeparturesAndArrivalsPeriodServiceJourneyApiModel**](VT.ApiPlaneraResa.Web.V4.Models.DeparturesAndArrivals.ServiceJourneyApiModel.md)> |  | [optional]
**stop_point** | [**crate::models::DeparturesAndArrivalsPeriodStopPointApiModel**](VT.ApiPlaneraResa.Web.V4.Models.DeparturesAndArrivals.StopPointApiModel.md) |  | 
**planned_time** | **String** | The planned time of the call in RFC 3339 format. | 
**estimated_time** | Option<**String**> | The estimated time of the call in RFC 3339 format. | [optional]
**estimated_otherwise_planned_time** | Option<**String**> | The best known time of the call in RFC 3339 format. Is EstimatedTime if exists, otherwise PlannedTime. | [optional][readonly]
**is_cancelled** | Option<**bool**> | Flag indicating if the departure or arrival is cancelled. | [optional]
**is_part_cancelled** | Option<**bool**> | Flag indicating if the departure or arrival is partially cancelled. | [optional]
**occupancy** | Option<[**crate::models::OccupancyInformationApiModel**](VT.ApiPlaneraResa.Web.V4.Models.OccupancyInformationApiModel.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


