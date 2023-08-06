# DeparturesAndArrivalsPeriodServiceJourneyDetailsApiModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**gid** | Option<**String**> | 16-digit Västtrafik service journey gid. | [optional]
**direction** | Option<**String**> | A description of the direction. | [optional]
**line** | Option<[**crate::models::DeparturesAndArrivalsPeriodLineDetailsApiModel**](VT.ApiPlaneraResa.Web.V4.Models.DeparturesAndArrivals.LineDetailsApiModel.md)> |  | [optional]
**service_journey_coordinates** | Option<[**Vec<crate::models::DeparturesAndArrivalsPeriodCoordinateApiModel>**](VT.ApiPlaneraResa.Web.V4.Models.DeparturesAndArrivals.CoordinateApiModel.md)> | The coordinates of the service journey. | [optional]
**calls_on_service_journey** | Option<[**Vec<crate::models::DeparturesAndArrivalsPeriodCallDetailsApiModel>**](VT.ApiPlaneraResa.Web.V4.Models.DeparturesAndArrivals.CallDetailsApiModel.md)> | All calls on the service journey. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


