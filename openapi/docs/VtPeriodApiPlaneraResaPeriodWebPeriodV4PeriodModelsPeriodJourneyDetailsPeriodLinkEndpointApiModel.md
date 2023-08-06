# JourneyDetailsPeriodLinkEndpointApiModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**gid** | Option<**String**> | The 16-digit Västtrafik gid. | [optional]
**name** | **String** | The location name. | 
**location_type** | [**crate::models::LocationType**](VT.ApiPlaneraResa.Core.Models.LocationType.md) |  | 
**latitude** | Option<**f64**> | The WGS84 latitude of the location. | [optional]
**longitude** | Option<**f64**> | The WGS84 longitude of the location. | [optional]
**planned_time** | **String** | The planned time in RFC 3339 format. | 
**estimated_time** | Option<**String**> | The estimated time in RFC 3339 format. | [optional]
**estimated_otherwise_planned_time** | Option<**String**> | The best known time of the link in RFC 3339 format. Is EstimatedTime if exists, otherwise PlannedTime. | [optional][readonly]
**notes** | Option<[**Vec<crate::models::Note>**](VT.ApiPlaneraResa.Core.Models.Note.md)> | An ordered list (most important first) of notes related to the end point. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


