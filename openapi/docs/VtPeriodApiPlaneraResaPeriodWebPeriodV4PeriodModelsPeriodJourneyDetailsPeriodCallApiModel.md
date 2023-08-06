# JourneyDetailsPeriodCallApiModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**stop_point** | [**crate::models::JourneyDetailsPeriodStopPointApiModel**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.StopPointApiModel.md) |  | 
**planned_time** | **String** | The planned time of the call in RFC 3339 format. | 
**estimated_time** | Option<**String**> | The estimated time of the call in RFC 3339 format. | [optional]
**estimated_otherwise_planned_time** | Option<**String**> | The best known time of the call in RFC 3339 format. Is EstimatedTime if exists, otherwise PlannedTime. | [optional][readonly]
**notes** | Option<[**Vec<crate::models::Note>**](VT.ApiPlaneraResa.Core.Models.Note.md)> | An ordered list (most important first) of notes related to the call. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


