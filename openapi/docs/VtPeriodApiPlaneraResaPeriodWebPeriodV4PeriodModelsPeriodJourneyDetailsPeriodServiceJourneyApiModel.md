# JourneyDetailsPeriodServiceJourneyApiModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**gid** | Option<**String**> | 16-digit Västtrafik service journey gid that the trip leg is a part of. | [optional]
**direction** | Option<**String**> | A description of the direction. | [optional]
**line** | Option<[**crate::models::JourneyDetailsPeriodLineDetailsApiModel**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.LineDetailsApiModel.md)> |  | [optional]
**service_journey_coordinates** | Option<[**Vec<crate::models::CoordinateApiModel>**](VT.ApiPlaneraResa.Web.V4.Models.CoordinateApiModel.md)> | The coordinates on the service journey. | [optional]
**calls_on_service_journey** | Option<[**Vec<crate::models::JourneyDetailsPeriodCallDetailsApiModel>**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.CallDetailsApiModel.md)> | All calls on the service journey. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


