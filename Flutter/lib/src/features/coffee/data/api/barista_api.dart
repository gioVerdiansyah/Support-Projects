
import 'package:coffe_shop/src/core/exceptions/api_exception.dart';
import 'package:coffe_shop/src/core/network/api_client.dart';
import 'package:coffe_shop/src/core/network/api_path.dart';
import 'package:coffe_shop/src/features/barista/data/models/barista_model.dart';

class BaristaApi {
  final ApiClient _api;

  BaristaApi(this._api);

  Future<BaristaModel> example() async {
    final results = await _api.get(ApiPath.example);

    return BaristaModel.fromJson(results.data);
  }
}